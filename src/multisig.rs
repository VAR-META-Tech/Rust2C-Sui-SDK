use super::SuiClientSingleton;
use anyhow::Ok;
use shared_crypto::intent::{Intent, IntentMessage};
use std::path::PathBuf;
use std::str::FromStr;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::TransactionData,
    },
};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::PublicKey;
use sui_types::crypto::Signature;
use sui_types::multisig::{MultiSig, MultiSigPublicKey, WeightUnit};
use sui_types::signature::GenericSignature;
use sui_types::transaction::{Argument, Command, Transaction};

pub fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}
pub async fn get_or_create_multisig_public_key(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<MultiSigPublicKey, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let mut pk_map: Vec<(PublicKey, WeightUnit)> = vec![];
    for (index, address) in addresses.iter().enumerate() {
        pk_map.push((
            keystore
                .get_key(&SuiAddress::from_str(address)?)?
                .public()
                .clone(),
            weights[index],
        ))
    }
    Ok(MultiSigPublicKey::insecure_new(pk_map, threshold))
}

pub async fn get_or_create_multisig_public_key_serialize(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<Vec<u8>, anyhow::Error> {
    Ok(bcs::to_bytes(
        &get_or_create_multisig_public_key(addresses, weights, threshold).await?,
    )?)
}

pub async fn get_or_create_multisig_address(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<String, anyhow::Error> {
    Ok(
        SuiAddress::from(&get_or_create_multisig_public_key(addresses, weights, threshold).await?)
            .to_string(),
    )
}

pub async fn create_sui_transaction(
    multisig_addr: &str,
    recipient_address: &str,
    amount: u64,
) -> Result<TransactionData, anyhow::Error> {
    let multisig_addr = SuiAddress::from_str(multisig_addr)?;
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let recipient = SuiAddress::from_str(recipient_address)?;

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
    let coins = sui_client
        .coin_read_api()
        .get_coins(multisig_addr, None, None, None) // get the first five coins
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();
    let builder = ptb.finish();
    let gas_budget = 5_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    Ok(TransactionData::new_programmable(
        multisig_addr,
        selected_gas_coins,
        builder,
        gas_budget,
        gas_price,
    ))
}

pub async fn _sign_and_execute_transaction(
    tx_data: Vec<u8>,
    signers_addresses: Vec<&str>,
    multisig_pk: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let tx_data: TransactionData = bcs::from_bytes(&tx_data)?;
    let multisig_pk: MultiSigPublicKey = bcs::from_bytes(&multisig_pk)?;
    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    let mut signatures = Vec::with_capacity(signers_addresses.len());
    for address in signers_addresses {
        signatures.push(
            GenericSignature::from(Signature::new_secure(
                &intent_msg,
                keystore.get_key(&SuiAddress::from_str(address)?)?,
            ))
            .to_compressed()
            .unwrap(),
        );
    }

    let multisig =
        GenericSignature::MultiSig(MultiSig::insecure_new(signatures, 0b011, multisig_pk));

    let tx = Transaction::from_generic_sig_data(tx_data, vec![multisig]);
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(tx, SuiTransactionBlockResponseOptions::default(), None)
        .await?;
    println!(
        "Transaction executed. Transaction digest: {}",
        transaction_response.digest.base58_encode()
    );
    Ok(())
}
