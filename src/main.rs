// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use fastcrypto::hash::HashFunction;
use fastcrypto::traits::{EncodeDecodeBase64, KeyPair};
use rand::Error;

use sui_keys::key_derive::generate_new_key;
use sui_sdk::SuiClientBuilder;
use tempfile::TempDir;

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, InMemKeystore, Keystore};
use sui_types::crypto::{DefaultHash, SignatureScheme, SuiKeyPair, SuiSignatureInner};
use sui_types::{
    base_types::{SuiAddress, SUI_ADDRESS_LENGTH},
    crypto::Ed25519SuiSignature,
};
mod balance;
mod wallet;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    balance::_get_all_balances(
        "0x0cc4b15265e0a342a2822377258e3750ecea621172e580395674790b33844a6b",
    )
    .await?;
    Ok(())
}
