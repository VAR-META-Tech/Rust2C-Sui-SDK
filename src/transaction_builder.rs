use std::{ffi::{c_char, CStr}, str::FromStr};

use anyhow::Result;
use shared_crypto::intent::Intent;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_types::{base_types::{ObjectID, SuiAddress}, programmable_transaction_builder::ProgrammableTransactionBuilder, quorum_driver_types::ExecuteTransactionRequestType, transaction::{Argument, Command, ProgrammableTransaction, Transaction, TransactionData}};

use crate::sui_client::SuiClientSingleton;

// #[repr(C)]
pub struct CProgrammableTransactionBuilder {
    builder: ProgrammableTransactionBuilder,
}

impl CProgrammableTransactionBuilder {
    pub fn new() -> Self {
        Self {
            builder: ProgrammableTransactionBuilder::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn create_builder() -> *mut CProgrammableTransactionBuilder {
    Box::into_raw(Box::new(CProgrammableTransactionBuilder::new()))
}

#[no_mangle]
pub extern "C" fn add_transfer_object_command(
    builder: *mut CProgrammableTransactionBuilder,
    recipient: *const c_char,
) {
    let builder = unsafe { &mut *builder };
    let recipient_str = unsafe { CStr::from_ptr(recipient).to_str().unwrap() };
    let recipient = SuiAddress::from_str(&recipient_str).unwrap();

    let argument_address = builder.builder.pure(recipient).unwrap();

    builder.builder.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
}

#[no_mangle]
pub extern "C" fn add_split_coins_command(
    builder: *mut CProgrammableTransactionBuilder,
    amount: u64,
) {
    let builder = unsafe { &mut *builder };
    let argument_amount = builder.builder.pure(amount).unwrap();

    builder.builder.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![argument_amount],
    ));
}

pub async fn _execute_transaction(
    sender: &str,
    transaction_data: ProgrammableTransaction
) -> Result<(SuiTransactionBlockResponse), anyhow::Error> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let sender_address = SuiAddress::from_str(sender)?;
    let coins = sui_client
        .coin_read_api()
        .get_coins(sender_address, None, None, None)
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();
    let gas_budget = 5_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender_address,
        selected_gas_coins,
        transaction_data,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender_address, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\n Transaction information: ");
    //return transaction_response;
    Ok(transaction_response)
}

//excute transaction
#[no_mangle]
pub extern "C" fn execute_transaction(
    builder: *mut CProgrammableTransactionBuilder,
    sender: *const c_char,
) -> *mut c_char {
    let builder = unsafe { Box::from_raw(builder) };
    let sender_str = unsafe { CStr::from_ptr(sender).to_str().unwrap() };

    let transaction_data = builder.builder.finish();
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move { _execute_transaction(&sender_str, transaction_data).await });

    let result_str = match result {
        Ok(response) => format!("{:?}", response),
        Err(e) => format!("Error: {:?}", e),
    };

    let c_string = std::ffi::CString::new(result_str).unwrap();
    c_string.into_raw()
}