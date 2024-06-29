// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use fastcrypto::encoding::Encoding;
use fastcrypto::hash::HashFunction;
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
use sui_types::crypto::{PublicKey, Signer};
use sui_types::crypto::{SignableBytes, ToFromBytes};
use sui_types::crypto::{Signature, SuiSignature};
use sui_types::multisig::{MultiSig, MultiSigPublicKey};
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

pub async fn multisig_transaction() -> Result<(), anyhow::Error> {
    let sui_client = SuiClientBuilder::default().build_devnet().await?;

    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());

    let key0 = keystore.get_key(&SuiAddress::from_str(
        "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
    )?)?;
    let key1 = keystore.get_key(&SuiAddress::from_str(
        "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733",
    )?)?;
    let key2 = keystore.get_key(&SuiAddress::from_str(
        "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75",
    )?)?;

    let pk0 = key0.public(); // ed25519
    let pk1 = key1.public(); // ed25519
    let pk2 = key2.public(); // ed25519

    let multisig_pk = MultiSigPublicKey::insecure_new(
        vec![(pk0.clone(), 1), (pk1.clone(), 1), (pk2.clone(), 1)],
        2,
    );
    let multisig_addr = SuiAddress::from(&multisig_pk);

    let address =
        SuiAddress::from_str("0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd")?;
    let recipient =
        SuiAddress::from_str("0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75")?;
    let amount: u64 = 540000000;
    // transactions::request_tokens_from_faucet(multisig_addr.to_string().as_str()).await?;
    let mut ptb2 = ProgrammableTransactionBuilder::new();
    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb2.pure(amount)?; // note that we need to specify the u64 type
    ptb2.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));
    // 3) transfer the new coin to a different address
    let argument_address = ptb2.pure(recipient)?;
    ptb2.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
    let gas_coin = sui_client
        .coin_read_api()
        .get_coins(multisig_addr, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No coins found for sender"))?;
    let builder2 = ptb2.finish();
    let gas_budget = 5_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    let tx_data4 = TransactionData::new_programmable(
        multisig_addr,
        vec![gas_coin.object_ref()],
        builder2,
        gas_budget,
        gas_price,
    );
    let bytes = bcs::to_bytes(&tx_data4)?;
    println!("1111111",);
    let tx_data6: TransactionData = bcs::from_bytes(&bytes)?;
    // let tx_data5 = TransactionData::from_signable_bytes(&bytes)?;
    println!("222222 ");

    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data4.clone());
    let signers = [&key0, &key1];
    let mut signatures = Vec::with_capacity(signers.len());
    for signer in signers {
        signatures.push(
            GenericSignature::from(Signature::new_secure(&intent_msg, *signer))
                .to_compressed()
                .unwrap(),
        );
    }
    let multisig =
        GenericSignature::MultiSig(MultiSig::insecure_new(signatures, 0b011, multisig_pk));

    let tx = Transaction::from_generic_sig_data(tx_data6, vec![multisig]);
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
