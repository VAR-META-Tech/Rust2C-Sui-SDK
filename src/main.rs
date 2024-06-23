// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod utils;
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::request_tokens_from_faucet;
use anyhow::anyhow;
use fastcrypto::encoding::Encoding;
use fastcrypto::hash::HashFunction;
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    encoding::Base64,
    secp256k1::Secp256k1KeyPair,
    secp256r1::Secp256r1KeyPair,
    traits::{EncodeDecodeBase64, KeyPair},
};
use rand::{rngs::StdRng, SeedableRng};
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
use sui_types::crypto::Signer;
use sui_types::crypto::SuiSignature;
use sui_types::crypto::ToFromBytes;
use sui_types::signature::GenericSignature;
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
/// This example walks through the Rust SDK use case described in
/// https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/sui-101/sign-and-send-txn.mdx
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // set up sui client for the desired network.
    let sui_client = SuiClientBuilder::default().build_devnet().await?;
    // let keypair =
    //     SuiKeyPair::decode_base64("AK9CMolzS+I").map_err(|_| anyhow!("Invalid base64"))?;
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());

    let address =
        SuiAddress::from_str("0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd")?;
    let keypair = keystore.get_key(&address)?;

    // replace `skp_determ_0` with the variable names above
    let pk = keypair.public();
    let sender = SuiAddress::from(&pk);
    println!("Sender: {:?}", sender);

    let gas_coin = sui_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No coins found for sender"))?;

    // construct an example programmable transaction.
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.pay_sui(vec![sender], vec![2])?;
        builder.finish()
    };

    let gas_budget = 5_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;

    // create the transaction data that will be sent to the network.
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin.object_ref()],
        pt,
        gas_budget,
        gas_price,
    );
    // recipient: SuiAddress,
    // sender: SuiAddress,
    // amount: Option<u64>,
    // gas_payment: ObjectRef,
    // gas_budget: u64,
    // gas_price: u64,
    let recipient =
        SuiAddress::from_str("0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75")
            .unwrap();
    let tx_data2 = TransactionData::new_transfer_sui(
        recipient,
        sender,
        Some(2000000000),
        gas_coin.object_ref(),
        gas_budget,
        gas_price,
    );

    // derive the digest that the keypair should sign on,
    // i.e. the blake2b hash of `intent || tx_data`.
    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data2);
    let raw_tx = bcs::to_bytes(&intent_msg).expect("bcs should not fail");
    let mut hasher = sui_types::crypto::DefaultHash::default();
    hasher.update(raw_tx.clone());
    let digest = hasher.finalize().digest;

    // use SuiKeyPair to sign the digest.
    let sui_sig = keypair.sign(&digest);

    // if you would like to verify the signature locally before submission, use this function.
    // if it fails to verify locally, the transaction will fail to execute in Sui.
    let res = sui_sig.verify_secure(
        &intent_msg,
        sender,
        sui_types::crypto::SignatureScheme::ED25519,
    );
    assert!(res.is_ok());

    // execute the transaction.
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            sui_types::transaction::Transaction::from_generic_sig_data(
                intent_msg.value,
                vec![GenericSignature::Signature(sui_sig)],
            ),
            SuiTransactionBlockResponseOptions::default(),
            None,
        )
        .await?;

    println!(
        "Transaction executed. Transaction digest: {}",
        transaction_response.digest.base58_encode()
    );
    println!("{transaction_response}");
    Ok(())
}
