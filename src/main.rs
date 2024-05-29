// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use fastcrypto::hash::HashFunction;
use fastcrypto::traits::{EncodeDecodeBase64, KeyPair};
use rand::Error;
use sui_keys::key_derive::generate_new_key;
use tempfile::TempDir;

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, InMemKeystore, Keystore};
use sui_types::crypto::{DefaultHash, SignatureScheme, SuiKeyPair, SuiSignatureInner};
use sui_types::{
    base_types::{SuiAddress, SUI_ADDRESS_LENGTH},
    crypto::Ed25519SuiSignature,
};

mod wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // wallet::get_wallet_from_address();
    // wallet::generate_new();
    // wallet::generate_and_add_key();
    // wallet::get_addresses();
    // wallet::get_keys();
    // wallet::import_from_mnemonic();
    // wallet::import_from_private_key();
    Ok(())
}
