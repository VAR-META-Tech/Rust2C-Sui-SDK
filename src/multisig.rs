use super::SuiClientSingleton;

use std::str::FromStr;
use std::{ffi::c_char, path::PathBuf};

use anyhow::{anyhow, Ok};
use fastcrypto::encoding::Encoding;
use fastcrypto::hash::HashFunction;
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use shared_crypto::intent::{Intent, IntentMessage};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::TransactionData,
    },
    SuiClientBuilder,
};
use sui_types::crypto::{PublicKey, Signer};
use sui_types::crypto::{SignableBytes, ToFromBytes};
use sui_types::crypto::{Signature, SuiSignature};
use sui_types::multisig::{MultiSig, MultiSigPublicKey, WeightUnit};
use sui_types::signature::GenericSignature;
use sui_types::transaction::{Argument, Command, Transaction};
use sui_types::{
    base_types::SuiAddress,
    crypto::{get_key_pair_from_rng, SuiKeyPair},
};

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

pub async fn multisig_transaction() -> Result<(), anyhow::Error> {
    let multisig_pk = get_or_create_multisig_public_key(
        vec![
            "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
            "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733",
            "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75",
        ],
        vec![1, 1, 1],
        2,
    )
    .await?;
    let multisig_addr = SuiAddress::from(&multisig_pk);
    let bytes = bcs::to_bytes(&multisig_pk)?;
    println!("Vec<u8>: {:?}", bytes);
    let tx_data = create_sui_transaction(
        multisig_addr.to_string().as_str(),
        "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75",
        51240000000,
    )
    .await?;

    let _ = _sign_and_execute_transaction(
        bcs::to_bytes(&tx_data)?,
        vec![
            "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
            "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733",
        ],
        bcs::to_bytes(&multisig_pk)?,
    )
    .await?;

    Ok(())
}
