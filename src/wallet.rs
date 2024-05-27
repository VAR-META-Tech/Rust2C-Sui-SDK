use anyhow::anyhow;
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

pub fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}

pub fn get_addresses() {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let addresses = keystore.addresses();
    println!("addresses {:?}", addresses);
}

pub fn get_keys() {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let keys = keystore.keys();
    println!("keys {:?}", keys);
}

pub fn generate_and_add_key() {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let (address, phrase, scheme) = keystore
        .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
        .unwrap();
    println!("address {:?}", address);
    println!("mnemonic {:?}", phrase);
    println!("scheme {:?}", scheme);
}

pub fn import_from_mnemonic() -> Result<(), anyhow::Error> {
    let phrase = "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());

    let suiAddress = keystore
        .import_from_mnemonic(phrase, SignatureScheme::ED25519, None)
        .unwrap();
    println!("suiAddress {:?}", suiAddress);

    Ok(())
}

pub fn import_from_private_key() -> Result<(), anyhow::Error> {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());

    let key_pair = SuiKeyPair::decode(
        "suiprivkey1qrpmsxplnrykppt542s7n2yy66a7dk7vrv3435h800r5c7xdr3mejhrsxwy",
    )
    .map_err(|_| anyhow!("Invalid Bech32"))?;
    let pub_key = key_pair.public();
    let alias = "111".to_string();
    let _ = keystore.add_key(Some(alias), key_pair);
    println!("pub_key {:?}", pub_key);
    Ok(())
}

pub fn get_wallet_from_address() -> Result<(), Box<dyn std::error::Error>> {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());

    let address =
        SuiAddress::from_str("0x50c699d21585f4ed024f9f69fac11bddede767cd376911cf2c9c50bd4801aa8f")?;
    println!("address {:?}", address);
    let key = keystore.get_key(&address)?;
    println!("key {:?}", key);
    println!("public {:?}", key.public().encode_base64());
    // Extract the private key from the keypair
    let private_key = match key {
        SuiKeyPair::Ed25519(kp) => kp.encode_base64(),
        SuiKeyPair::Secp256k1(kp) => kp.encode_base64(),
        SuiKeyPair::Secp256r1(kp) => kp.encode_base64(),
    };
    println!("private_key {:?}", key.public().encode_base64());

    Ok(())
}
