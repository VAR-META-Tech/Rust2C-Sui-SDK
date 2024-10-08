use crate::sui_client::SuiClientSingleton;
use anyhow::Result;
use anyhow::{anyhow, Ok};
use move_core_types::language_storage::StructTag;
use shared_crypto::intent::Intent;
use std::ffi::{c_char, CStr, CString};
use std::str::FromStr;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{
    SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponseQuery,
};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ProgrammableMoveCall, Transaction, TransactionData,
        },
        Identifier,
    },
};
use sui_types::base_types::SuiAddress;
use sui_types::transaction::ObjectArg;
use tokio::runtime;

pub async fn _mint(
    package_id: &str,
    sender_address: &str,
    name: &str,
    description: &str,
    uri: &str,
) -> Result<(), anyhow::Error> {
    // 1) get the Sui client, the sender and recipient that we will use
    // for the transaction, and find the coin we use as gas
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let sender: SuiAddress = SuiAddress::from_str(sender_address)?;

    // we need to find the coin we will use as gas
    let coins = sui_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    // 2) create a programmable transaction builder to add commands and create a PTB
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Create an Argument::Input for Pure 6 value of type u64
    let input_value = name;
    let input_argument = CallArg::Pure(bcs::to_bytes(&input_value).unwrap());

    // Add this input to the builder
    ptb.input(input_argument)?;

    let input_value = description;
    let input_argument = CallArg::Pure(bcs::to_bytes(&input_value).unwrap());

    // Add this input to the builder
    ptb.input(input_argument)?;
    let input_value = uri;
    let input_argument = CallArg::Pure(bcs::to_bytes(&input_value).unwrap());

    // Add this input to the builder
    ptb.input(input_argument)?;

    // 3) add a move call to the PTB
    // Replace the pkg_id with the package id you want to call
    let package = ObjectID::from_hex_literal(package_id).map_err(|e| anyhow!(e))?;
    let module = Identifier::new("nft").map_err(|e| anyhow!(e))?;
    let function = Identifier::new("mint_to_sender").map_err(|e| anyhow!(e))?;
    ptb.command(Command::move_call(
        package,
        module,
        function,
        vec![],
        vec![Argument::Input(0), Argument::Input(1), Argument::Input(2)],
    ));

    // build the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let gas_budget = 10_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

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
    println!("{}", transaction_response);
    Ok(())
}

pub async fn _transfer_nft(
    package_id: &str,
    sender_address: &str,
    nft_id: &str,
    recipient_address: &str,
) -> Result<(), anyhow::Error> {
    // 1) get the Sui client, the sender and recipient that we will use
    // for the transaction, and find the coin we use as gas
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let sender: SuiAddress = SuiAddress::from_str(sender_address)?;
    // we need to find the coin we will use as gas
    let coins = sui_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    let mut ptb = ProgrammableTransactionBuilder::new();

    let owned_objects = sui_client
        .read_api()
        .get_owned_objects(sender, None, None, None)
        .await?;
    let nft_object_info = owned_objects
        .data
        .iter()
        .find(|obj| obj.object_id().unwrap() == ObjectID::from_str(nft_id).unwrap())
        .ok_or_else(|| anyhow!("NFT object not found"))?;

    let object_ref = <std::option::Option<SuiObjectData> as Clone>::clone(&nft_object_info.data)
        .unwrap()
        .object_ref();
    // Convert inputs to CallArg
    let nft_id_argument = CallArg::Object(ObjectArg::ImmOrOwnedObject(object_ref));
    let recipient_argument = CallArg::Pure(
        bcs::to_bytes(&SuiAddress::from_str(recipient_address).map_err(|e| anyhow!(e))?).unwrap(),
    );
    ptb.input(nft_id_argument)?;
    ptb.input(recipient_argument)?;
    // 3) add a move call to the PTB
    // Replace the pkg_id with the package id you want to call
    let package = ObjectID::from_hex_literal(package_id).map_err(|e| anyhow!(e))?;
    let module = Identifier::new("nft").map_err(|e| anyhow!(e))?;
    let function = Identifier::new("transfer").map_err(|e| anyhow!(e))?;
    ptb.command(Command::move_call(
        package,
        module,
        function,
        vec![],
        vec![Argument::Input(0), Argument::Input(1)],
    ));

    // build the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let gas_budget = 10_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

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
    println!("{}", transaction_response);
    Ok(())
}

pub async fn _get_wallet_objects(address: &str, object_type: &str) -> Result<Vec<SuiObjectData>> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let active_address: SuiAddress = SuiAddress::from_str(address)?;
    let query = Some(SuiObjectResponseQuery {
        filter: Some(SuiObjectDataFilter::StructType(StructTag::from_str(
            object_type,
        )?)),
        options: Some(SuiObjectDataOptions::new().with_type().with_content()),
    });
    let owned_objects = sui_client
        .read_api()
        .get_owned_objects(active_address, query, None, None)
        .await?
        .data;
    let data: Vec<SuiObjectData> = owned_objects
        .into_iter()
        .filter_map(|owned_objects| owned_objects.data)
        .collect();
    Ok(data)
}

#[no_mangle]
pub extern "C" fn mint_nft(
    package_id: *const c_char,
    sender_address: *const c_char,
    name: *const c_char,
    description: *const c_char,
    uri: *const c_char,
) -> *const c_char {
    let c_str = unsafe {
        assert!(!package_id.is_null());
        CStr::from_ptr(package_id)
    };
    let package_id = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address)
    };
    let sender_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!description.is_null());
        CStr::from_ptr(description)
    };
    let description = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!uri.is_null());
        CStr::from_ptr(uri)
    };
    let uri = c_str.to_str().unwrap_or("Invalid UTF-8");
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _mint(package_id, sender_address, name, description, uri).await {
            Result::Ok(()) => {
                let success_message = CString::new("Mint NFT to sender success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn transfer_nft(
    package_id: *const c_char,
    sender_address: *const c_char,
    nft_id: *const c_char,
    recipient_address: *const c_char,
) -> *const c_char {
    let c_str = unsafe {
        assert!(!package_id.is_null());
        CStr::from_ptr(package_id)
    };
    let package_id = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address)
    };
    let sender_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!nft_id.is_null());
        CStr::from_ptr(nft_id)
    };
    let nft_id = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!recipient_address.is_null());
        CStr::from_ptr(recipient_address)
    };
    let recipient_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _transfer_nft(package_id, sender_address, nft_id, recipient_address).await {
            Result::Ok(()) => {
                let success_message = CString::new("Transfer NFT success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}