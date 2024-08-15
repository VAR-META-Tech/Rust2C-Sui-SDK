use anyhow::{anyhow, Ok};
use fastcrypto::traits::EncodeDecodeBase64;
use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use std::ptr;
use std::str::FromStr;
use sui_keys::key_derive::generate_new_key;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{SignatureScheme, SuiKeyPair};

#[repr(C)]
pub struct Wallet {
    address: *mut c_char,
    mnemonic: *mut c_char,
    public_base64_key: *mut c_char,
    private_key: *mut c_char,
    key_scheme: *mut c_char,
}

impl Wallet {
    fn from_generate_result(
        address: SuiAddress,
        kp: SuiKeyPair,
        scheme: SignatureScheme,
        phrase: String,
    ) -> Wallet {
        Wallet {
            address: Wallet::string_to_c_char(Some(address.to_string())),
            mnemonic: Wallet::string_to_c_char(Some(phrase)),
            public_base64_key: Wallet::string_to_c_char(Some(kp.public().encode_base64())),
            private_key: Wallet::string_to_c_char(Some(kp.encode_base64())),
            key_scheme: Wallet::string_to_c_char(Some(scheme.to_string())),
        }
    }
    fn from_generate_and_add_new_key(
        address: SuiAddress,
        scheme: SignatureScheme,
        phrase: String,
    ) -> Wallet {
        Wallet {
            address: Wallet::string_to_c_char(Some(address.to_string())),
            mnemonic: Wallet::string_to_c_char(Some(phrase)),
            public_base64_key: Wallet::string_to_c_char(None),
            private_key: Wallet::string_to_c_char(None),
            key_scheme: Wallet::string_to_c_char(Some(scheme.to_string())),
        }
    }
    fn new(
        address: Option<String>,
        mnemonic: Option<String>,
        public_base64_key: Option<String>,
        private_key: Option<String>,
        key_scheme: Option<String>,
    ) -> Wallet {
        Wallet {
            address: Wallet::string_to_c_char(address),
            mnemonic: Wallet::string_to_c_char(mnemonic),
            public_base64_key: Wallet::string_to_c_char(public_base64_key),
            private_key: Wallet::string_to_c_char(private_key),
            key_scheme: Wallet::string_to_c_char(key_scheme),
        }
    }

    fn string_to_c_char(s: Option<String>) -> *mut c_char {
        match s {
            Some(str) => CString::new(str).unwrap().into_raw(),
            None => ptr::null_mut(),
        }
    }
    pub fn show(&self) {
        let address = unsafe { CStr::from_ptr(self.address) }
            .to_str()
            .unwrap_or("Not set");
        let mnemonic = unsafe { CStr::from_ptr(self.mnemonic) }
            .to_str()
            .unwrap_or("Not set");
        let public_base64_key = unsafe { CStr::from_ptr(self.public_base64_key) }
            .to_str()
            .unwrap_or("Not set");
        let private_key = unsafe { CStr::from_ptr(self.private_key) }
            .to_str()
            .unwrap_or("Not set");
        let key_scheme = unsafe { CStr::from_ptr(self.key_scheme) }
            .to_str()
            .unwrap_or("Not set");

        println!("Wallet Address: {}", address);
        println!("Mnemonic: {}", mnemonic);
        println!("Public Base64 Key: {}", public_base64_key);
        println!("Private Key: {}", private_key);
        println!("Key Scheme: {}", key_scheme);
    }
    pub fn free(&mut self) {
        unsafe {
            if !self.address.is_null() {
                let _ = CString::from_raw(self.address);
                self.address = ptr::null_mut();
            }
            if !self.mnemonic.is_null() {
                let _ = CString::from_raw(self.mnemonic);
                self.mnemonic = ptr::null_mut();
            }
            if !self.public_base64_key.is_null() {
                let _ = CString::from_raw(self.public_base64_key);
                self.public_base64_key = ptr::null_mut();
            }
            if !self.private_key.is_null() {
                let _ = CString::from_raw(self.private_key);
                self.private_key = ptr::null_mut();
            }
            if !self.key_scheme.is_null() {
                let _ = CString::from_raw(self.key_scheme);
                self.key_scheme = ptr::null_mut();
            }
        }
    }
}

pub fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}

pub fn get_addresses() {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let addresses = keystore.addresses();
    println!("addresses {:?}", addresses);
}

pub fn get_keys() {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let keys = keystore.keys();
    println!("keys {:?}", keys);
}

pub fn get_wallets() -> Result<Vec<Wallet>, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let addresses = keystore.addresses();
    let mut wallets: Vec<Wallet> = Vec::new();
    for address in addresses.iter() {
        wallets.push(get_wallet_from_address(address.to_string().as_str()).unwrap())
    }
    Ok(wallets)
}

pub fn generate_and_add_key() -> Result<Wallet, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let (address, phrase, scheme) = keystore
        .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
        .unwrap();
    Ok(Wallet::from_generate_and_add_new_key(
        address, scheme, phrase,
    ))
}

pub fn import_from_mnemonic(mnemonic: &str) -> Result<String, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let _sui_addresss = keystore
        .import_from_mnemonic(mnemonic, SignatureScheme::ED25519, None)
        .unwrap();

    Ok(_sui_addresss.to_string())
}

pub fn import_from_private_key(key_base64: &str) -> Result<(), anyhow::Error> {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let key_pair = SuiKeyPair::decode_base64(key_base64).map_err(|_| anyhow!("Invalid base64"))?;
    let _ = keystore.add_key(None, key_pair);
    Ok(())
}

pub fn get_wallet_from_address(address: &str) -> Result<Wallet, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let address = SuiAddress::from_str(address)?;
    let key = keystore.get_key(&address)?;
    let scheme = match key {
        SuiKeyPair::Ed25519(_) => SignatureScheme::ED25519,
        SuiKeyPair::Secp256k1(_) => SignatureScheme::Secp256k1,
        SuiKeyPair::Secp256r1(_) => SignatureScheme::Secp256r1,
    };
    Ok(Wallet::new(
        Some(address.to_string()),
        None,
        Some(key.public().encode_base64()),
        Some(key.encode_base64()),
        Some(scheme.to_string()),
    ))
}

pub fn generate_new(key_scheme: &str, word_length: &str) -> Result<Wallet, anyhow::Error> {
    let scheme = match key_scheme.to_lowercase().as_str() {
        "ed25519" => Ok(SignatureScheme::ED25519),
        "secp256k1" => Ok(SignatureScheme::Secp256k1),
        "secp256r1" => Ok(SignatureScheme::Secp256r1),
        "bls12381" => Ok(SignatureScheme::BLS12381),
        "multisig" => Ok(SignatureScheme::MultiSig),
        "zkloginauthenticator" => Ok(SignatureScheme::ZkLoginAuthenticator),
        _ => Ok(SignatureScheme::ED25519),
    }
    .unwrap();
    let _word_length = match word_length.to_lowercase().as_str() {
        "word12" => Ok("word12"),
        "word15" => Ok("word15"),
        "word18" => Ok("word18"),
        "word21" => Ok("word21"),
        "word24" => Ok("word24"),
        _ => Ok("word12"),
    }
    .unwrap();
    let (address, kp, scheme, phrase) =
        generate_new_key(scheme, None, Some(_word_length.to_string()))?;
    Ok(Wallet::from_generate_result(address, kp, scheme, phrase))
}

