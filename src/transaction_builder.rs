use std::{
    ffi::{c_char, c_longlong, c_ulonglong, CStr},
    str::FromStr,
};

use anyhow::Result;
use shared_crypto::intent::Intent;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{Coin, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    quorum_driver_types::ExecuteTransactionRequestType,
    transaction::{
        Argument, Command, ProgrammableMoveCall, ProgrammableTransaction, Transaction,
        TransactionData,
    },
    Identifier, TypeTag,
};

use crate::{c_types::CPure, sui_client::SuiClientSingleton};

// #[repr(C)]
pub struct CProgrammableTransactionBuilder {
    builder: ProgrammableTransactionBuilder,
}

pub struct CTypeTags {
    tag: Vec<TypeTag>,
}

impl CTypeTags {
    pub fn new() -> Self {
        Self { tag: Vec::new() }
    }
}

pub struct CArguments {
    arguments: Vec<Argument>,
}

impl CArguments {
    pub fn new() -> Self {
        Self {
            arguments: Vec::new(),
        }
    }
}

impl CProgrammableTransactionBuilder {
    pub fn new() -> Self {
        Self {
            builder: ProgrammableTransactionBuilder::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn create_type_tags() -> *mut CTypeTags {
    Box::into_raw(Box::new(CTypeTags::new()))
}

#[no_mangle]
pub extern "C" fn add_type_tag(type_tags: *mut CTypeTags, tag: *const c_char) {
    let type_tags = unsafe { &mut *type_tags };
    let tag_str = unsafe { CStr::from_ptr(tag).to_str().unwrap() };
    let tag = TypeTag::from_str(tag_str).unwrap();
    type_tags.tag.push(tag);
}

#[no_mangle]
pub extern "C" fn destroy_type_tags(type_tags: *mut CTypeTags) {
    unsafe {
        Box::from_raw(type_tags);
    }
}

#[no_mangle]
pub extern "C" fn create_arguments() -> *mut CArguments {
    Box::into_raw(Box::new(CArguments::new()))
}

#[no_mangle]
pub extern "C" fn destroy_arguments(arguments: *mut CArguments) {
    unsafe {
        Box::from_raw(arguments);
    }
}

#[no_mangle]
pub extern "C" fn add_argument_gas_coin(arguments: *mut CArguments) {
    let arguments = unsafe { &mut *arguments };
    arguments.arguments.push(Argument::GasCoin);
}

#[no_mangle]
pub extern "C" fn add_argument_result(arguments: *mut CArguments, value: u16) {
    let arguments = unsafe { &mut *arguments };
    arguments.arguments.push(Argument::Result(value));
}

#[no_mangle]
pub extern "C" fn add_argument_input(arguments: *mut CArguments, value: u16) {
    let arguments = unsafe { &mut *arguments };
    arguments.arguments.push(Argument::Input(value));
}

#[no_mangle]
pub extern "C" fn add_argument_nested_result(arguments: *mut CArguments, value1: u16, value2: u16) {
    let arguments = unsafe { &mut *arguments };
    arguments
        .arguments
        .push(Argument::NestedResult(value1, value2));
}

#[no_mangle]
pub extern "C" fn make_pure(
    builder: *mut CProgrammableTransactionBuilder,
    arguments: *mut CArguments,
    value: *mut CPure,
) {
    let builder = unsafe { &mut *builder };
    let arguments = unsafe { &mut *arguments };
    let value = unsafe { &*value };
    let argument = builder.builder.pure_bytes(value.data.clone(), false);
    arguments.arguments.push(argument);
}

#[no_mangle]
pub extern "C" fn create_builder() -> *mut CProgrammableTransactionBuilder {
    Box::into_raw(Box::new(CProgrammableTransactionBuilder::new()))
}

#[no_mangle]
pub extern "C" fn destroy_builder(builder: *mut CProgrammableTransactionBuilder) {
    unsafe {
        Box::from_raw(builder);
    }
}

#[no_mangle]
pub extern "C" fn add_move_call_command(
    builder: *mut CProgrammableTransactionBuilder,
    package: *const c_char,
    module: *const c_char,
    function: *const c_char,
    type_arguments: *mut CTypeTags,
    arguments: *mut CArguments,
) {
    let builder = unsafe { &mut *builder };
    let package_str = unsafe { CStr::from_ptr(package).to_str().unwrap() };
    let module_str = unsafe { CStr::from_ptr(module).to_str().unwrap() };
    let function_str = unsafe { CStr::from_ptr(function).to_str().unwrap() };
    let type_tags = unsafe { &*type_arguments };
    let arguments = unsafe { &*arguments };

    let package = ObjectID::from_hex_literal(package_str).map_err(|e| anyhow::Error::msg(e));
    let module = Identifier::new(module_str).map_err(|e| anyhow::Error::msg(e));
    let function = Identifier::new(function_str).map_err(|e| anyhow::Error::msg(e));

    builder.builder.command(Command::move_call(
        package.unwrap(),
        module.unwrap(),
        function.unwrap(),
        type_tags.tag.clone(),
        arguments.arguments.clone(),
    ));
}

#[no_mangle]
pub extern "C" fn add_transfer_object_command(
    builder: *mut CProgrammableTransactionBuilder,
    agreements: *mut CArguments,
    recipient: *mut CArguments,
) {
    let builder = unsafe { &mut *builder };
    let agreements = unsafe { &*agreements }.arguments.clone();
    let recipient = unsafe { &*recipient }.arguments.clone();

    builder
        .builder
        .command(Command::TransferObjects(agreements, recipient[0]));
}

#[no_mangle]
pub extern "C" fn add_split_coins_command(
    builder: *mut CProgrammableTransactionBuilder,
    coin: *mut CArguments,
    agreements: *mut CArguments,
) {
    let builder = unsafe { &mut *builder };
    let coin = unsafe { &*coin }.arguments.clone();
    let agreements = unsafe { &*agreements }.arguments.clone();

    builder
        .builder
        .command(Command::SplitCoins(coin[0], agreements));
}

#[no_mangle]
pub extern "C" fn add_merge_coins_command(
    builder: *mut CProgrammableTransactionBuilder,
    coin: *mut CArguments,
    agreements: *mut CArguments,
) {
    let builder = unsafe { &mut *builder };
    let coin = unsafe { &*coin }.arguments.clone();
    let agreements = unsafe { &*agreements }.arguments.clone();

    builder
        .builder
        .command(Command::MergeCoins(coin[0], agreements));
}

pub async fn _execute_transaction(
    sender: &str,
    transaction_data: ProgrammableTransaction,
    gas_budget: u64,
) -> Result<(SuiTransactionBlockResponse), anyhow::Error> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let sender_address = SuiAddress::from_str(sender)?;
    let coins = sui_client
        .coin_read_api()
        .get_coins(sender_address, None, None, None)
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();
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
    if !transaction_response.status_ok().unwrap_or(false) {
        return Err(anyhow::Error::msg("Transaction failed"));
    }
    Ok(transaction_response)
}

//excute transaction
#[no_mangle]
pub extern "C" fn execute_transaction(
    builder: *mut CProgrammableTransactionBuilder,
    sender: *const c_char,
    gas_budget: c_ulonglong,
) -> *mut c_char {
    let builder = unsafe { Box::from_raw(builder) };
    let sender_str = unsafe { CStr::from_ptr(sender).to_str().unwrap() };

    let transaction_data = builder.builder.finish();
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(
            async move { _execute_transaction(&sender_str, transaction_data, gas_budget).await },
        );

    let result_str = match result {
        Ok(response) => format!("{:?}", response),
        Err(e) => format!("Error: {:?}", e),
    };

    let c_string = std::ffi::CString::new(result_str).unwrap();
    c_string.into_raw()
}

pub async fn _execute_transaction_allow_sponser(
    sender: &str,
    transaction_data: ProgrammableTransaction,
    gas_budget: u64,
    sponser: &str,
) -> Result<(SuiTransactionBlockResponse), anyhow::Error> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let sender_address = SuiAddress::from_str(sender)?;
    let sponser_address = SuiAddress::from_str(sponser)?;
    let coins = sui_client
        .coin_read_api()
        .get_coins(sponser_address, None, None, None)
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable_allow_sponsor(
        sender_address,
        selected_gas_coins,
        transaction_data,
        gas_budget,
        gas_price,
        sponser_address,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&sender_address, &tx_data, Intent::sui_transaction())?;
    let sponser_signature =
        keystore.sign_secure(&sponser_address, &tx_data, Intent::sui_transaction())?;

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature, sponser_signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    print!("done\n Transaction information: ");
    if !transaction_response.status_ok().unwrap_or(false) {
        return Err(anyhow::Error::msg("Transaction failed"));
    }
    //return transaction_response;
    Ok(transaction_response)
}

#[no_mangle]
pub extern "C" fn execute_transaction_allow_sponser(
    builder: *mut CProgrammableTransactionBuilder,
    sender: *const c_char,
    gas_budget: c_ulonglong,
    sponser: *const c_char,
) -> *mut c_char {
    let builder = unsafe { Box::from_raw(builder) };
    let sender_str = unsafe { CStr::from_ptr(sender).to_str().unwrap() };
    let sponser_str = unsafe { CStr::from_ptr(sponser).to_str().unwrap() };

    let transaction_data = builder.builder.finish();
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            _execute_transaction_allow_sponser(
                &sender_str,
                transaction_data,
                gas_budget,
                &sponser_str,
            )
            .await
        });

    let result_str = match result {
        Ok(response) => format!("{:?}", response),
        Err(e) => format!("Error: {:?}", e),
    };

    let c_string = std::ffi::CString::new(result_str).unwrap();
    c_string.into_raw()
}
