use anyhow::{anyhow, Ok};
use fastcrypto::traits::EncodeDecodeBase64;
use std::ffi::{c_char, c_int, CStr, CString};
use std::path::PathBuf;
use std::str::FromStr;
use std::{ptr, result};
use sui_keys::key_derive::generate_new_key;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{SignatureScheme, SuiKeyPair};

use crate::c_types::{CSuiObjectData, CSuiObjectDataArray};
use crate::nfts::_get_wallet_objects;

//Public functions for FFI

#[repr(C)]
pub struct Wallet {
    address: *mut c_char,
    mnemonic: *mut c_char,
    public_base64_key: *mut c_char,
    private_key: *mut c_char,
    key_scheme: *mut c_char,
}

#[repr(C)]
pub struct ImportResult {
    status: c_int,
    address: *mut c_char,
    error: *mut c_char,
}

#[repr(C)]
pub enum ResultStatus {
    Success = 0,
    Error,
}

#[repr(C)]
pub struct WalletList {
    wallets: *mut Wallet,
    length: usize,
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

#[no_mangle]
pub extern "C" fn get_wallets() -> WalletList {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let addresses = keystore.addresses();
    // make WalletList from addresses
    let wallets: Vec<Wallet> = addresses
        .iter()
        .map(|address| get_wallet_from_address_private(&address.to_string()).unwrap())
        .collect();
    let wallets_len = wallets.len();
    let wallet_list = WalletList {
        wallets: Box::into_raw(wallets.into_boxed_slice()) as *mut Wallet,
        length: wallets_len,
    };
    wallet_list
}

#[no_mangle]
pub extern "C" fn free_wallet_list(wallet_list: WalletList) {
    if !wallet_list.wallets.is_null() {
        // Convert the pointer back into a Box to drop it
        unsafe {
            let _ = Box::from_raw(wallet_list.wallets);
        }
    }
}

#[no_mangle]
pub extern "C" fn free_wallet(wallet: *mut Wallet) {
    if !wallet.is_null() {
        // Convert the pointer back into a Box to drop it
        unsafe {
            let _ = Box::from_raw(wallet);
        }
    }
}

#[no_mangle]
pub extern "C" fn generate_wallet(
    key_scheme: *const c_char,
    word_length: *const c_char,
) -> *mut Wallet {
    let c_key_scheme = unsafe {
        assert!(!key_scheme.is_null());
        CStr::from_ptr(key_scheme)
    };
    let key_scheme_str = c_key_scheme.to_str().unwrap_or("Invalid UTF-8");

    let c_word_length = unsafe {
        assert!(!word_length.is_null());
        CStr::from_ptr(word_length)
    };
    let word_length_str = c_word_length.to_str().unwrap_or("Invalid UTF-8");
    let wallet = generate_new_private(key_scheme_str, word_length_str).unwrap();
    Box::into_raw(Box::new(wallet))
}

#[no_mangle]
pub extern "C" fn generate_and_add_key() -> *mut Wallet {
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let (address, phrase, scheme) = keystore
        .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
        .unwrap();
    Box::into_raw(Box::new(Wallet::from_generate_and_add_new_key(
        address, scheme, phrase,
    ))) as *mut Wallet
}

#[no_mangle]
pub extern "C" fn import_from_mnemonic(
    mnemonic: *const c_char,
    sig_scheme: *const c_char,
    alias: *const c_char,
) -> *mut ImportResult {
    let c_mnemonic = unsafe {
        assert!(!mnemonic.is_null());
        CStr::from_ptr(mnemonic)
    };

    let c_sig_scheme = unsafe {
        assert!(!sig_scheme.is_null());
        CStr::from_ptr(sig_scheme)
    };

    let c_alias = unsafe {
        assert!(!alias.is_null());
        CStr::from_ptr(alias)
    };

    //maping sigScheme_str to SignatureScheme
    let signature_scheme;
    if c_sig_scheme.to_str().unwrap_or("") == "ed25519" {
        signature_scheme = SignatureScheme::ED25519;
    } else if c_sig_scheme.to_str().unwrap_or("") == "secp256k1" {
        signature_scheme = SignatureScheme::Secp256k1;
    } else if c_sig_scheme.to_str().unwrap_or("") == "secp256r1" {
        signature_scheme = SignatureScheme::Secp256r1;
    } else {
        signature_scheme = SignatureScheme::ED25519;
    }
    let alias_string = c_alias.to_str().unwrap_or("").to_string();
    let alias = if alias_string == "" {
        None
    } else {
        Some(alias_string)
    };
    let mnemonic = c_mnemonic.to_str().unwrap_or("");
    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let result = match keystore.import_from_mnemonic(mnemonic, signature_scheme, None, alias) {
        result::Result::Ok(sui_address) => ImportResult {
            status: ResultStatus::Success as c_int,
            address: Wallet::string_to_c_char(Some(sui_address.to_string())),
            error: Wallet::string_to_c_char(None),
        },
        result::Result::Err(e) => ImportResult {
            status: ResultStatus::Error as c_int,
            address: Wallet::string_to_c_char(None),
            error: Wallet::string_to_c_char(Some(e.to_string())),
        },
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn import_from_private_key(key_base64: *const c_char) -> *mut ImportResult {
    let key_base64_str = unsafe {
        assert!(!key_base64.is_null());
        CStr::from_ptr(key_base64)
    };

    let keystore_path = default_keystore_path();
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let key_pair = match SuiKeyPair::decode_base64(key_base64_str.to_str().unwrap_or("")) {
        result::Result::Ok(key_pair) => key_pair,
        Err(err) => {
            return Box::into_raw(Box::new(ImportResult {
                status: ResultStatus::Error as c_int,
                address: Wallet::string_to_c_char(None),
                error: Wallet::string_to_c_char(Some(err.to_string())),
            }));
        }
    };
    //get address from keypair
    let address = SuiAddress::from(&key_pair.public());
    let result = keystore.add_key(None, key_pair);
    let result = match result {
        //Ok return wallet address
        result::Result::Ok(_) => ImportResult {
            status: ResultStatus::Success as c_int,
            address: Wallet::string_to_c_char(Some(address.to_string())),
            error: Wallet::string_to_c_char(None),
        },
        //Err return error from add_key
        result::Result::Err(e) => ImportResult {
            status: ResultStatus::Error as c_int,
            address: Wallet::string_to_c_char(None),
            error: Wallet::string_to_c_char(Some(e.to_string())),
        },
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn get_wallet_from_address(address: *const c_char) -> *mut Wallet {
    // Safely convert the C string to a Rust string
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("");
    // Get wallet from address and return null if error
    let wallet = match get_wallet_from_address_private(address_str) {
        result::Result::Ok(wallet) => wallet,
        Err(_) => Wallet::new(None, None, None, None, None),
    };
    Box::into_raw(Box::new(wallet))
}

//Private functions

fn get_wallet_from_address_private(address: &str) -> Result<Wallet, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(
        FileBasedKeystore::new(&keystore_path)
            .map_err(|e| anyhow!("Failed to create keystore: {}", e))?,
    );

    // Get address from string or return error without crashing
    let address = SuiAddress::from_str(address).map_err(|_| anyhow!("Invalid address"))?;
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

fn generate_new_private(key_scheme: &str, word_length: &str) -> Result<Wallet, anyhow::Error> {
    let scheme = match key_scheme.to_lowercase().as_str() {
        "ed25519" => Ok(SignatureScheme::ED25519),
        "secp256k1" => Ok(SignatureScheme::Secp256k1),
        "secp256r1" => Ok(SignatureScheme::Secp256r1),
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

fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}

#[no_mangle]
pub extern "C" fn get_wallet_objects(
    address: *const c_char,
    object_type: *const c_char,
) -> CSuiObjectDataArray {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!object_type.is_null());
        CStr::from_ptr(object_type)
    };
    let object_type = c_str.to_str().unwrap_or("Invalid UTF-8");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let objects = runtime
        .block_on(_get_wallet_objects(address, object_type))
        .unwrap_or_else(|_| Vec::new());
    let mut c_objects: Vec<CSuiObjectData> = objects
        .into_iter()
        .map(|obj| CSuiObjectData::from(obj))
        .collect();
    let ptr = c_objects.as_mut_ptr();
    let len = c_objects.len();
    std::mem::forget(c_objects);
    CSuiObjectDataArray { data: ptr, len }
}
