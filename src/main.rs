mod utils;
use std::str::FromStr;

use anyhow::anyhow;
use fastcrypto::encoding::Base58;
use shared_crypto::intent::Intent;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{SuiObjectData, SuiObjectRef};
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
    SuiClientBuilder,
};
use sui_types::{
    base_types::{ObjectRef, SequenceNumber, SuiAddress},
    digests::ObjectDigest,
    object::{Object, ObjectRead},
    transaction::ObjectArg,
};
use utils::setup_for_write;

// This example shows how to use programmable transactions to chain multiple
// commands into one transaction, and specifically how to call a function from a move package
// These are the following steps:
// 1) finds a coin from the active address that has Sui,
// 2) creates a PTB and adds an input to it,
// 3) adds a move call to the PTB,
// 4) signs the transaction,
// 5) executes it.
// For some of these actions it prints some output.
// Finally, at the end of the program it prints the number of coins for the
// Sui address that received the coin.
// If you run this program several times, you should see the number of coins
// for the recipient address increases.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui_client = SuiClientBuilder::default().build_devnet().await?;
    let active_address: SuiAddress =
        SuiAddress::from_str("0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd")?;

    // we need to find the coin we will use as gas
    let coins = sui_client
        .coin_read_api()
        .get_coins(active_address, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    let mut ptb = ProgrammableTransactionBuilder::new();

    let nft_id = "0x9dcd002c78d7408563d86c82f4dec8aa2e6653c64c46ff85f92131e6ec89aa71";
    let recipient = "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75";

    let owned_objects = sui_client
        .read_api()
        .get_owned_objects(active_address, None, None, None)
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
        bcs::to_bytes(&SuiAddress::from_str(recipient).map_err(|e| anyhow!(e))?).unwrap(),
    );
    ptb.input(nft_id_argument)?;
    ptb.input(recipient_argument)?;
    // 3) add a move call to the PTB
    // Replace the pkg_id with the package id you want to call
    let pkg_id = "0xd1efbd86210322b550a8d6017ad5113fda2bd4f486593096f83e7b9ce3cbd002";
    let package = ObjectID::from_hex_literal(pkg_id).map_err(|e| anyhow!(e))?;
    let module = Identifier::new("nft").map_err(|e| anyhow!(e))?;
    let function = Identifier::new("transfer").map_err(|e| anyhow!(e))?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module,
        function,
        type_arguments: vec![],
        arguments: vec![Argument::Input(0), Argument::Input(1)],
    })));

    // build the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    let gas_budget = 10_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        active_address,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(&active_address, &tx_data, Intent::sui_transaction())?;

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
