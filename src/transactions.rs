// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use c_types::{CStringArray, CU8Array};
use futures::{future, stream::StreamExt};
use reqwest::Client;
use serde_json::json;
use shared_crypto::intent::Intent;
use std::{
    ffi::{c_char, c_uint, CStr, CString},
    slice,
    str::FromStr,
    time::Duration,
};
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{Coin, SuiObjectDataOptions};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Argument, Command, Transaction, TransactionData},
    },
    SuiClient, SuiClientBuilder,
};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    executable_transaction,
};
use tokio::runtime;

use crate::{
    c_types,
    multisig::{_sign_and_execute_transaction, create_sui_transaction},
    nfts::{_mint, _transfer_nft},
    transaction_builder::CProgrammableTransactionBuilder,
};
const SUI_FAUCET: &str = "https://faucet.devnet.sui.io/gas";

pub async fn _programmable_transaction(
    senderaddress: &str,
    recipientaddress: &str,
    amount: u64,
) -> Result<(), anyhow::Error> {
    // 1) get the Sui client, the sender and recipient that we will use
    // for the transaction, and find the coin we use as gas

    let sender = SuiAddress::from_str(senderaddress)?;
    let recipient = SuiAddress::from_str(recipientaddress)?;
    let sui = SuiClientBuilder::default().build_devnet().await?;
    let _coin = fetch_coin(&sui, &sender).await?;
    if _coin.is_none() {
        _request_tokens_from_faucet(senderaddress).await?;
    }
    // we need to find the coin we will use as gas
    let coins = sui
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();

    // programmable transactions allows the user to bundle a number of actions into one transaction
    let mut ptb = ProgrammableTransactionBuilder::new();

    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb.pure(amount)?; // note that we need to specify the u64 type
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));

    // 3) transfer the new coin to a different address
    let argument_address = ptb.pure(recipient)?;
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));

    // finish building the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let gas_budget = 5_000_000;
    let gas_price = sui.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        selected_gas_coins,
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\n Transaction information: ");
    println!("{:?}", transaction_response);

    let coins = sui
        .coin_read_api()
        .get_coins(recipient, None, None, None)
        .await?;

    println!(
        "After the transfer, the recipient address {recipient} has {} coins",
        coins.data.len()
    );
    Ok(())
}

pub async fn _programmable_transaction_allow_sponser(
    senderaddress: &str,
    recipientaddress: &str,
    amount: u64,
    sponser_address: &str,
) -> Result<(), anyhow::Error> {
    // 1) get the Sui client, the sender and recipient that we will use
    // for the transaction, and find the coin we use as gas

    let sponser = SuiAddress::from_str(sponser_address)?;
    let sender = SuiAddress::from_str(senderaddress)?;
    let recipient = SuiAddress::from_str(recipientaddress)?;
    let sui = SuiClientBuilder::default().build_devnet().await?;
    let _coin = fetch_coin(&sui, &sender).await?;
    if _coin.is_none() {
        _request_tokens_from_faucet(senderaddress).await?;
    }
    // we need to find the coin we will use as gas
    let coins = sui
        .coin_read_api()
        .get_coins(sponser, None, None, None)
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();

    // programmable transactions allows the user to bundle a number of actions into one transaction
    let mut ptb = ProgrammableTransactionBuilder::new();

    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb.pure(amount)?; // note that we need to specify the u64 type
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));

    // 3) transfer the new coin to a different address
    let argument_address = ptb.pure(recipient)?;
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));

    // finish building the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let gas_budget = 5_000_000;
    let gas_price = sui.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable_allow_sponsor(
        sender,
        selected_gas_coins,
        builder,
        gas_budget,
        gas_price,
        sponser,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let sender_signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;
    let sponser_signature = keystore.sign_secure(&sponser, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![sender_signature, sponser_signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\n Transaction information: ");
    println!("{:?}", transaction_response);

    let coins = sui
        .coin_read_api()
        .get_coins(recipient, None, None, None)
        .await?;

    println!(
        "After the transfer, the recipient address {recipient} has {} coins",
        coins.data.len()
    );
    Ok(())
}

/// Request tokens from the Faucet for the given address
#[allow(unused_assignments)]
pub async fn _request_tokens_from_faucet(address_str: &str) -> Result<(), anyhow::Error> {
    let json_body = json![{
        "FixedAmountRequest": {
            "recipient": &address_str
        }
    }];
    // make the request to the faucet JSON RPC API for coin
    let client = Client::new();
    let resp = client
        .post(SUI_FAUCET)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;
    println!(
        "_Faucet request for address {address_str} has status: {}",
        resp.status()
    );

    Ok(())
}

/// Return the coin owned by the address that has at least 5_000_000 MIST, otherwise returns None
pub async fn fetch_coin(
    sui: &SuiClient,
    sender: &SuiAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::sui::SUI".to_string();
    let coins_stream = sui
        .coin_read_api()
        .get_coins_stream(*sender, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();
    let coin = coins.next().await;
    Ok(coin)
}

//Public functions for FFI
#[no_mangle]
pub extern "C" fn create_transaction(
    from_address: *const c_char,
    to_address: *const c_char,
    amount: u64,
) -> c_types::CU8Array {
    let rt = runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!from_address.is_null());
        CStr::from_ptr(from_address)
    };
    let from_address = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!to_address.is_null());
        CStr::from_ptr(to_address)
    };
    let to_address = c_str.to_str().unwrap_or("Invalid UTF-8");
    rt.block_on(async {
        match create_sui_transaction(from_address, to_address, amount).await {
            Ok(tx) => {
                let bytes = bcs::to_bytes(&tx).unwrap();
                println!("Vec<u8> transaction in Rust: {:?}", bytes);
                let boxed_bytes = bytes.into_boxed_slice();
                let data_ptr = boxed_bytes.as_ptr();
                let len = boxed_bytes.len() as c_uint;
                std::mem::forget(boxed_bytes);
                c_types::CU8Array {
                    data: data_ptr,
                    len: len,
                    error: std::ptr::null(),
                }
            } // Return 0 to indicate success.
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                c_types::CU8Array {
                    data: std::ptr::null(),
                    len: 0,
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn programmable_transaction(
    sender_address: *const c_char,
    recipient_address: *const c_char,
    amount: u64,
) -> *const c_char {
    // Convert C strings to Rust strings
    let sender = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address).to_string_lossy().to_string()
    };
    let recipient = unsafe {
        assert!(!recipient_address.is_null());
        CStr::from_ptr(recipient_address)
            .to_string_lossy()
            .to_string()
    };

    // Run the async function synchronously
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move { _programmable_transaction(&sender, &recipient, amount).await });

    // Return the result as a C string
    match result {
        Ok(_) => CString::new("Transaction completed successfully")
            .unwrap()
            .into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn programmable_transaction_allow_sponser(
    sender_address: *const c_char,
    recipient_address: *const c_char,
    amount: u64,
    sponser_address: *const c_char,
) -> *const c_char {
    // Convert C strings to Rust strings
    let sender = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address).to_string_lossy().to_string()
    };
    let recipient = unsafe {
        assert!(!recipient_address.is_null());
        CStr::from_ptr(recipient_address)
            .to_string_lossy()
            .to_string()
    };
    let sponser = unsafe {
        assert!(!sponser_address.is_null());
        CStr::from_ptr(sponser_address)
            .to_string_lossy()
            .to_string()
    };

    // Run the async function synchronously
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            _programmable_transaction_allow_sponser(&sender, &recipient, amount, &sponser).await
        });

    // Return the result as a C string
    match result {
        Ok(_) => CString::new("Transaction completed successfully")
            .map(|cstr| cstr.into_raw())
            .unwrap_or(std::ptr::null_mut()),
        Err(e) => CString::new(format!("Error: {}", e))
            .map(|cstr| cstr.into_raw())
            .unwrap_or(std::ptr::null_mut()),
    }
}

#[no_mangle]
pub extern "C" fn request_tokens_from_faucet(address_str: *const c_char) -> *const c_char {
    let address = unsafe { CStr::from_ptr(address_str).to_string_lossy().to_string() };

    // Run the async function synchronously inside the Rust environment
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { _request_tokens_from_faucet(&address).await });

    match result {
        Ok(_) => CString::new("Request successful").unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}
