use nfts::{_get_wallet_objects, _mint, _transfer_nft};
use serde::de;
use std::ffi::{c_char, c_int, CString};
use std::ffi::{c_uchar, c_uint, CStr};
use std::{ptr, slice};
use sui_client::{
    _api_version, _available_rpc_methods, _available_subscriptions, _check_api_version,
};
use sui_json_rpc_types::{Page, SuiData, SuiObjectData};
use sui_sdk::{SuiClient, SuiClientBuilder};
// Import the necessary crates
use anyhow::{anyhow, Result};
use sui_types::base_types::SuiAddress;
use tokio::runtime; // Using Tokio as the async runtime

mod balance;
mod coin_read_api;
mod multisig;
mod nfts;
mod sui_client;
mod transactions;
mod utils;
mod wallet;
use balance::get_all_balances;
use balance::get_balance;
use balance::get_coins;
use balance::get_total_supply;
use coin_read_api::_coin_read_api;
use multisig::{
    _sign_and_execute_transaction, create_sui_transaction, get_or_create_multisig_public_key,
};

use std::collections::HashMap;
use transactions::{request_tokens_from_faucet, ProgrammableTransactionAllowSponser};
use transactions::ProgrammableTransaction;
use wallet::Wallet;
mod event_api;
use event_api::_event_api;
use once_cell::sync::OnceCell;
use sui_client::{connect_devnet, connect_localnet, connect_testnet};
use sui_json_rpc_types::Balance;
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum SuiEnvironment {
    Testnet,
    Devnet,
}

struct SuiClientSingleton {
    client: Mutex<Option<SuiClient>>,
    environment: Mutex<Option<SuiEnvironment>>,
}

impl SuiClientSingleton {
    fn instance() -> &'static SuiClientSingleton {
        static INSTANCE: OnceCell<SuiClientSingleton> = OnceCell::new();
        INSTANCE.get_or_init(|| SuiClientSingleton {
            client: Mutex::new(None),
            environment: Mutex::new(None),
        })
    }

    async fn initialize(&self, environment: SuiEnvironment) -> Result<()> {
        let mut env_guard = self.environment.lock().await;
        if env_guard.is_some() {
            return Err(anyhow!("Environment already initialized"));
        }
        *env_guard = Some(environment);
        Ok(())
    }

    async fn get_or_init(&self) -> Result<SuiClient> {
        let mut env_guard = self.environment.lock().await;
        let environment = if let Some(env) = &*env_guard {
            env.clone()
        } else {
            let default_env = SuiEnvironment::Devnet;
            *env_guard = Some(default_env.clone());
            default_env
        };

        let mut client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            Ok(client.clone())
        } else {
            let client = match environment {
                SuiEnvironment::Testnet => SuiClientBuilder::default().build_testnet().await?,
                SuiEnvironment::Devnet => SuiClientBuilder::default().build_devnet().await?,
            };
            *client_guard = Some(client.clone());
            Ok(client)
        }
    }
}

pub async fn _test() -> Result<()> {
    let sui_singleton = SuiClientSingleton::instance();

    // Retrieve the singleton instance of SuiClient
    let sui_client = sui_singleton.get_or_init().await?;
    println!("SuiClient initialized.");

    // If called again, it will return the cached instance
    let sui_client_cached = sui_singleton.get_or_init().await?;
    println!("SuiClient retrieved from cache.");

    println!("Sui testnet version is: {}", sui_client.api_version());

    Ok(())
}

#[no_mangle]
pub extern "C" fn test() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _test().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

pub async fn _buildTestnet() -> Result<()> {
    let sui_client_singleton = SuiClientSingleton::instance();

    // Initialize environment only once
    match sui_client_singleton
        .initialize(SuiEnvironment::Testnet)
        .await
    {
        Ok(()) => println!("Environment initialized to Testnet."),
        Err(e) => eprintln!("Failed to initialize environment: {:?}", e),
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn buildTestnet() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _buildTestnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

pub async fn _buildDevnet() -> Result<()> {
    let sui_client_singleton = SuiClientSingleton::instance();

    // Initialize environment only once
    match sui_client_singleton
        .initialize(SuiEnvironment::Devnet)
        .await
    {
        Ok(()) => println!("Environment initialized to Devnet."),
        Err(e) => eprintln!("Failed to initialize environment: {:?}", e),
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn buildDevnet() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _buildDevnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}
//Wallet

#[repr(C)]
pub struct WalletList {
    wallets: *mut Wallet,
    length: usize,
}

#[no_mangle]
pub extern "C" fn get_wallets() -> WalletList {
    let wallets = wallet::get_wallets().unwrap();

    let mut wallets = wallets.into_boxed_slice();
    let wallet_ptr = wallets.as_mut_ptr();
    let length = wallets.len();
    std::mem::forget(wallets);

    WalletList {
        wallets: wallet_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn free_wallet_list(wallet_list: WalletList) {
    unsafe {
        let wallets =
            Vec::from_raw_parts(wallet_list.wallets, wallet_list.length, wallet_list.length);
        for mut wallet in wallets {
            wallet.free();
        }
    }
}
#[no_mangle]
pub extern "C" fn free_wallet(wallet: *mut Wallet) {
    if !wallet.is_null() {
        let mut wallet = unsafe { Box::from_raw(wallet) };
        wallet.free();
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
    let wallet = wallet::generate_new(key_scheme_str, word_length_str).unwrap();
    Box::into_raw(Box::new(wallet))
}

#[no_mangle]
pub extern "C" fn generate_and_add_key() -> *mut Wallet {
    let wallet = wallet::generate_and_add_key().unwrap();
    Box::into_raw(Box::new(wallet))
}
#[repr(C)]
pub struct CSuiObjectData {
    object_id: *mut c_char,
    version: u64,
    digest: *mut c_char,
    type_: *mut c_char,
    owner: *mut c_char,
    previous_transaction: *mut c_char,
    storage_rebate: u64,
    display: *mut c_char,
    content: *mut c_char,
    bcs: *mut c_char,
}

impl CSuiObjectData {
    fn from(data: SuiObjectData) -> Self {
        let content = data
            .content
            .unwrap()
            .try_as_move()
            .unwrap()
            .fields
            .clone()
            .to_json_value()
            .to_string();
        CSuiObjectData {
            object_id: CString::new(data.object_id.to_string()).unwrap().into_raw(),
            version: data.version.value(),
            digest: CString::new(data.digest.to_string()).unwrap().into_raw(),
            type_: match data.type_ {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            owner: match data.owner {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            previous_transaction: match data.previous_transaction {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            storage_rebate: data.storage_rebate.unwrap_or_default(),
            display: CString::new(format!("{:?}", data.display))
                .unwrap()
                .into_raw(),
            content: CString::new(content).unwrap().into_raw(),
            bcs: CString::new(format!("{:?}", data.bcs)).unwrap().into_raw(),
        }
    }
    pub fn show(&self) {
        unsafe {
            println!("object_id: {}", self.c_str_to_string(self.object_id));
            println!("version: {}", self.version);
            println!("digest: {}", self.c_str_to_string(self.digest));
            println!("type_: {}", self.c_str_to_string(self.type_));
            println!("owner: {}", self.c_str_to_string(self.owner));
            println!(
                "previous_transaction: {}",
                self.c_str_to_string(self.previous_transaction)
            );
            println!("storage_rebate: {}", self.storage_rebate);
            println!("display: {}", self.c_str_to_string(self.display));
            println!("content: {}", self.c_str_to_string(self.content));
            println!("bcs: {}", self.c_str_to_string(self.bcs));
        }
    }

    unsafe fn c_str_to_string(&self, c_str: *mut c_char) -> String {
        if c_str.is_null() {
            String::from("null")
        } else {
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }
}

#[repr(C)]
pub struct CSuiObjectDataArray {
    data: *mut CSuiObjectData,
    len: usize,
}
#[no_mangle]
pub extern "C" fn get_wallet_from_address(address: *const c_char) -> *mut Wallet {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let wallet = wallet::get_wallet_from_address(address_str).unwrap();
    Box::into_raw(Box::new(wallet))
}

#[no_mangle]
pub extern "C" fn import_from_private_key(key_base64: *const c_char) {
    let c_str = unsafe {
        assert!(!key_base64.is_null());
        CStr::from_ptr(key_base64)
    };
    let key_base64_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let _ = wallet::import_from_private_key(key_base64_str);
}

#[no_mangle]
pub extern "C" fn import_from_mnemonic(mnemonic: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!mnemonic.is_null());
        CStr::from_ptr(mnemonic)
    };
    let mnemonic_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let _address = wallet::import_from_mnemonic(mnemonic_str).unwrap();
    CString::new(_address).unwrap().into_raw()
}

// Struct to hold C-compatible string array
#[repr(C)]
pub struct CStringArray {
    data: *const *const c_char,
    len: c_int,
}

#[repr(C)]
pub struct CU8Array {
    data: *const c_uchar,
    len: c_uint,
    error: *const c_char,
}

#[repr(C)]
pub struct MultiSig {
    address: *const c_char,
    bytes: CU8Array,
    error: *const c_char,
}

#[no_mangle]
pub extern "C" fn free_multisig(multisig: MultiSig) {
    unsafe {
        if !multisig.address.is_null() {
            let _ = CString::from_raw(multisig.address as *mut c_char);
        }
        if !multisig.error.is_null() {
            let _ = CString::from_raw(multisig.error as *mut c_char);
        }
        if !multisig.bytes.data.is_null() {
            let _ = Box::from_raw(slice::from_raw_parts_mut(
                multisig.bytes.data as *mut u8,
                multisig.bytes.len as usize,
            ));
        }
    }
}

// Struct to hold the result, either CStringArray or error message
#[repr(C)]
pub struct ResultCStringArray {
    strings: CStringArray,
    error: *const c_char,
}

// Function to free the C-compatible string array
#[no_mangle]
pub extern "C" fn free_strings(array: CStringArray) {
    unsafe {
        for i in 0..array.len {
            let c_str_ptr = *array.data.add(i as usize);
            if !c_str_ptr.is_null() {
                CString::from_raw(c_str_ptr as *mut c_char);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn get_or_create_multisig(
    addresses: CStringArray,
    weights: CU8Array,
    threshold: u16,
) -> MultiSig {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        let addresses: Vec<&str> = unsafe {
            (0..addresses.len)
                .map(|i| {
                    let c_str = CStr::from_ptr(*addresses.data.add(i as usize));
                    c_str.to_str().expect("Invalid UTF-8")
                })
                .collect()
        };
        let weights: Vec<u8> =
            unsafe { slice::from_raw_parts(weights.data, weights.len as usize).to_vec() };

        match get_or_create_multisig_public_key(addresses, weights, threshold).await {
            Ok(multisig_pk) => {
                let bytes = bcs::to_bytes(&multisig_pk).unwrap();
                println!("Vec<u8> in Rust: {:?}", bytes);

                let boxed_bytes = bytes.into_boxed_slice();
                let data_ptr = boxed_bytes.as_ptr();
                let len = boxed_bytes.len() as c_uint;

                // Leak the boxed slice to keep it alive
                std::mem::forget(boxed_bytes);
                MultiSig {
                    bytes: CU8Array {
                        data: data_ptr,
                        len: len,
                        error: std::ptr::null(),
                    },
                    address: CString::new(SuiAddress::from(&multisig_pk).to_string())
                        .unwrap()
                        .into_raw(),
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                MultiSig {
                    bytes: CU8Array {
                        data: std::ptr::null(),
                        len: 0,
                        error: std::ptr::null(),
                    },
                    address: std::ptr::null(),
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn sign_and_execute_transaction(
    multisig: CU8Array,
    tx: CU8Array,
    addresses: CStringArray,
) -> *const c_char {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        let addresses: Vec<&str> = unsafe {
            (0..addresses.len)
                .map(|i| {
                    let c_str = CStr::from_ptr(*addresses.data.add(i as usize));
                    c_str.to_str().expect("Invalid UTF-8")
                })
                .collect()
        };
        let tx: Vec<u8> = unsafe { slice::from_raw_parts(tx.data, tx.len as usize).to_vec() };
        let multisig: Vec<u8> =
            unsafe { slice::from_raw_parts(multisig.data, multisig.len as usize).to_vec() };
        match _sign_and_execute_transaction(tx, addresses, multisig).await {
            Ok(()) => {
                let success_message = CString::new("Sign and execute transaction success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn mint_nft(
    package_id: *const c_char,
    sender_address: *const c_char,
    name: *const c_char,
    description: *const c_char,
    uri: *const c_char,
) -> *const c_char {
    let c_str = unsafe {
        assert!(!package_id.is_null());
        CStr::from_ptr(package_id)
    };
    let package_id = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address)
    };
    let sender_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!description.is_null());
        CStr::from_ptr(description)
    };
    let description = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!uri.is_null());
        CStr::from_ptr(uri)
    };
    let uri = c_str.to_str().unwrap_or("Invalid UTF-8");
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _mint(package_id, sender_address, name, description, uri).await {
            Ok(()) => {
                let success_message = CString::new("Mint NFT to sender success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn transfer_nft(
    package_id: *const c_char,
    sender_address: *const c_char,
    nft_id: *const c_char,
    recipient_address: *const c_char,
) -> *const c_char {
    let c_str = unsafe {
        assert!(!package_id.is_null());
        CStr::from_ptr(package_id)
    };
    let package_id = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!sender_address.is_null());
        CStr::from_ptr(sender_address)
    };
    let sender_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!nft_id.is_null());
        CStr::from_ptr(nft_id)
    };
    let nft_id = c_str.to_str().unwrap_or("Invalid UTF-8");

    let c_str = unsafe {
        assert!(!recipient_address.is_null());
        CStr::from_ptr(recipient_address)
    };
    let recipient_address = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _transfer_nft(package_id, sender_address, nft_id, recipient_address).await {
            Ok(()) => {
                let success_message = CString::new("Transfer NFT success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}

// Function to free the error string
#[no_mangle]
pub extern "C" fn free_error_string(error: *const c_char) {
    if !error.is_null() {
        unsafe {
            CString::from_raw(error as *mut c_char);
        }
    }
}

#[no_mangle]
pub extern "C" fn available_rpc_methods() -> ResultCStringArray {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _available_rpc_methods().await {
            Ok(strings) => {
                let mut c_strings: Vec<*const c_char> = strings
                    .into_iter()
                    .map(|s| CString::new(s).unwrap().into_raw() as *const c_char)
                    .collect();

                c_strings.shrink_to_fit();
                let data = c_strings.as_ptr();
                let len = c_strings.len() as c_int;

                // Prevent Rust from freeing the CString pointers
                std::mem::forget(c_strings);

                ResultCStringArray {
                    strings: CStringArray { data, len },
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                ResultCStringArray {
                    strings: CStringArray {
                        data: std::ptr::null(),
                        len: 0,
                    },
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn available_subscriptions() -> ResultCStringArray {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _available_subscriptions().await {
            Ok(strings) => {
                let mut c_strings: Vec<*const c_char> = strings
                    .into_iter()
                    .map(|s| CString::new(s).unwrap().into_raw() as *const c_char)
                    .collect();

                c_strings.shrink_to_fit();
                let data = c_strings.as_ptr();
                let len = c_strings.len() as c_int;

                // Prevent Rust from freeing the CString pointers
                std::mem::forget(c_strings);

                ResultCStringArray {
                    strings: CStringArray { data, len },
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                ResultCStringArray {
                    strings: CStringArray {
                        data: std::ptr::null(),
                        len: 0,
                    },
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn check_api_version() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _check_api_version().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn api_version() -> *const c_char {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _api_version().await {
            Ok(version) => version, // Return 0 to indicate success.
            Err(_) => CString::new("Error").unwrap().into_raw(), // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn create_transaction(
    from_address: *const c_char,
    to_address: *const c_char,
    amount: u64,
) -> CU8Array {
    let rt = runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!from_address.is_null());
        CStr::from_ptr(from_address)
    };
    let from_address = c_str.to_str().unwrap_or("Invalid UTF-8");
    let c_str = unsafe {
        assert!(!to_address.is_null());
        CStr::from_ptr(to_address)
    };
    let to_address = c_str.to_str().unwrap_or("Invalid UTF-8");
    rt.block_on(async {
        match create_sui_transaction(from_address, to_address, amount).await {
            Ok(tx) => {
                let bytes = bcs::to_bytes(&tx).unwrap();
                println!("Vec<u8> transaction in Rust: {:?}", bytes);
                let boxed_bytes = bytes.into_boxed_slice();
                let data_ptr = boxed_bytes.as_ptr();
                let len = boxed_bytes.len() as c_uint;
                std::mem::forget(boxed_bytes);
                CU8Array {
                    data: data_ptr,
                    len: len,
                    error: std::ptr::null(),
                }
            } // Return 0 to indicate success.
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                CU8Array {
                    data: std::ptr::null(),
                    len: 0,
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn connect_localnet_c() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match connect_localnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn connect_devnet_c() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match connect_devnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn connect_testnet_c() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match connect_testnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn coin_read_api() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();

    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _coin_read_api().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn event_api() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();

    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _event_api().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn get_total_supply_sync() -> u64 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(get_total_supply());
    match result {
        Ok(supply) => supply.value,
        Err(_) => 0, // Return 0 in case of error
    }
}

// C-compatible Balance struct
#[repr(C)]
pub struct CBalance {
    coin_type: *const c_char,
    coin_object_count: usize,
    total_balance: [u64; 2],
}

// C-compatible vector of balances
#[repr(C)]
pub struct CBalanceArray {
    balances: *const CBalance,
    length: usize,
}

// impl Balance {
//     fn to_c_balance(&self) -> CBalance {
//         let total_balance_bytes = self.total_balance.to_le_bytes();
//         CBalance {
//             coin_type: CString::new(self.coin_type.clone()).unwrap().into_raw(),
//             coin_object_count: self.coin_object_count,
//             total_balance: [
//                 u64::from_le_bytes(total_balance_bytes[0..8].try_into().unwrap()),
//                 u64::from_le_bytes(total_balance_bytes[8..16].try_into().unwrap()),
//             ],
//         }
//     }
// }

#[no_mangle]
pub extern "C" fn get_balance_sync(address: *const c_char) -> CBalance {
    // This is a placeholder function that simulates fetching a Balance
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let balance = runtime
        .block_on(get_balance(address_str))
        .unwrap_or_else(|_| Balance {
            coin_type: "".to_string(),
            coin_object_count: 0,
            total_balance: 0,
            locked_balance: HashMap::new(),
        });
    let total_balance_bytes = balance.total_balance.to_le_bytes();
    CBalance {
        coin_type: CString::new(balance.coin_type.clone()).unwrap().into_raw(),
        coin_object_count: balance.coin_object_count,
        total_balance: [
            u64::from_le_bytes(total_balance_bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(total_balance_bytes[8..16].try_into().unwrap()),
        ],
    }
}

#[no_mangle]
pub extern "C" fn free_balance(balance: CBalance) {
    if !balance.coin_type.is_null() {
        unsafe {
            drop(CString::from_raw(balance.coin_type as *mut c_char));
        }
    }
}

/// Wrapper for the Balance struct to implement methods
pub struct BalanceWrapper(Balance);

impl BalanceWrapper {
    fn to_c_balance(&self) -> CBalance {
        let total_balance_bytes = self.0.total_balance.to_le_bytes();
        CBalance {
            coin_type: CString::new(self.0.coin_type.clone()).unwrap().into_raw(),
            coin_object_count: self.0.coin_object_count,
            total_balance: [
                u64::from_le_bytes(total_balance_bytes[0..8].try_into().unwrap()),
                u64::from_le_bytes(total_balance_bytes[8..16].try_into().unwrap()),
            ],
        }
    }
}
// Function to convert a vector of Balances to a CBalanceArray
fn to_c_balance_array(balances: Vec<Balance>) -> CBalanceArray {
    let c_balances: Vec<CBalance> = balances
        .iter()
        .map(|b| BalanceWrapper(b.clone()).to_c_balance())
        .collect();
    let length = c_balances.len();
    let balances_ptr = c_balances.as_ptr();
    std::mem::forget(c_balances); // Prevent Rust from freeing the memory
    CBalanceArray {
        balances: balances_ptr,
        length,
    }
}

// Function to free a CBalanceArray
#[no_mangle]
pub extern "C" fn free_balance_array(balance_array: CBalanceArray) {
    if !balance_array.balances.is_null() {
        unsafe {
            let balances_slice = std::slice::from_raw_parts_mut(
                balance_array.balances as *mut CBalance,
                balance_array.length,
            );
            for balance in balances_slice.iter() {
                if !balance.coin_type.is_null() {
                    drop(CString::from_raw(balance.coin_type as *mut c_char));
                }
            }
            drop(Vec::from_raw_parts(
                balance_array.balances as *mut CBalance,
                balance_array.length,
                balance_array.length,
            ));
        }
    }
}

#[no_mangle]
pub extern "C" fn get_all_balances_sync(address: *const c_char) -> CBalanceArray {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let balances = runtime
        .block_on(get_all_balances(address_str))
        .unwrap_or_else(|_| Vec::new());
    to_c_balance_array(balances)
}

#[no_mangle]
pub extern "C" fn get_balances(address: *const c_char) -> CBalanceArray {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let balances = runtime
        .block_on(balance::_get_all_balances(address_str))
        .unwrap_or_else(|_| Vec::new());
    to_c_balance_array(balances)
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
    let mut c_objects: Vec<CSuiObjectData> =
        objects.into_iter().map(CSuiObjectData::from).collect();
    let ptr = c_objects.as_mut_ptr();
    let len = c_objects.len();
    std::mem::forget(c_objects);
    CSuiObjectDataArray { data: ptr, len }
}

#[no_mangle]
pub extern "C" fn free_sui_object_data_list(array: CSuiObjectDataArray) {
    if array.data.is_null() {
        return;
    }
    unsafe {
        let boxed_slice = Box::from_raw(std::slice::from_raw_parts_mut(array.data, array.len));
        for obj in boxed_slice.iter() {
            if !obj.object_id.is_null() {
                CString::from_raw(obj.object_id);
            }
            if !obj.digest.is_null() {
                CString::from_raw(obj.digest);
            }
            if !obj.type_.is_null() {
                CString::from_raw(obj.type_);
            }
            if !obj.owner.is_null() {
                CString::from_raw(obj.owner);
            }
            if !obj.previous_transaction.is_null() {
                CString::from_raw(obj.previous_transaction);
            }
            if !obj.display.is_null() {
                CString::from_raw(obj.display);
            }
            if !obj.content.is_null() {
                CString::from_raw(obj.content);
            }
            if !obj.bcs.is_null() {
                CString::from_raw(obj.bcs);
            }
        }
    }
}

// Wrapper struct for Coin
pub struct WrappedCoin {
    pub inner: sui_json_rpc_types::Coin,
}

fn string_to_c_char(s: Option<String>) -> *mut c_char {
    match s {
        Some(str) => CString::new(str).unwrap().into_raw(),
        None => ptr::null_mut(),
    }
}
impl WrappedCoin {
    pub fn to_c_coin(&self) -> CCoin {
        CCoin {
            coin_type: CString::new(self.inner.coin_type.clone())
                .unwrap()
                .into_raw(),
            coin_object_id: string_to_c_char(Some(self.inner.coin_object_id.to_string())),
            version: self.inner.version.value(),
            digest: string_to_c_char(Some(self.inner.digest.base58_encode())),
            balance: self.inner.balance,
            previous_transaction: string_to_c_char(Some(
                self.inner.previous_transaction.base58_encode(),
            )),
        }
    }
}

// C-compatible structures
#[repr(C)]
pub struct CCoin {
    coin_type: *mut c_char,
    coin_object_id: *mut c_char, // Changed to array of 32 bytes
    version: u64,
    digest: *mut c_char,
    balance: u64,
    previous_transaction: *mut c_char,
}

#[repr(C)]
pub struct CCoinArray {
    coins: *const CCoin,
    length: usize,
}

// Function to convert a vector of WrappedCoins to a CCoinArray
fn to_c_coin_array(coins: Vec<WrappedCoin>) -> CCoinArray {
    let c_coins: Vec<CCoin> = coins.iter().map(|c| c.to_c_coin()).collect();
    let length = c_coins.len();
    let coins_ptr = c_coins.as_ptr();
    std::mem::forget(c_coins); // Prevent Rust from freeing the memory
    CCoinArray {
        coins: coins_ptr,
        length,
    }
}

// Function to free a CCoinArray
#[no_mangle]
pub extern "C" fn free_coin_array(coin_array: CCoinArray) {
    if !coin_array.coins.is_null() {
        unsafe {
            drop(Vec::from_raw_parts(
                coin_array.coins as *mut CCoin,
                coin_array.length,
                coin_array.length,
            ));
        }
    }
}

// Synchronous wrapper to call the async get_coins function
#[no_mangle]
pub extern "C" fn get_coins_sync(address: *const c_char) -> CCoinArray {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let coins = runtime
        .block_on(get_coins(address_str))
        .unwrap_or_else(|_| Page {
            data: Vec::new(),
            next_cursor: None,
            has_next_page: false,
        });

    let wrapped_coins: Vec<WrappedCoin> = coins
        .data
        .into_iter()
        .map(|inner| WrappedCoin { inner })
        .collect();
    to_c_coin_array(wrapped_coins)
}

#[no_mangle]
pub extern "C" fn programmable_transaction(
    sender_address: *const c_char,
    recipient_address: *const c_char,
    amount: u64,
) -> *const c_char {
    let sender = unsafe { CStr::from_ptr(sender_address).to_string_lossy().to_string() };
    let recipient = unsafe {
        CStr::from_ptr(recipient_address)
            .to_string_lossy()
            .to_string()
    };

    // Here we run the async block synchronously for simplicity
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move { ProgrammableTransaction(&sender, &recipient, amount).await });

    match result {
        Ok(_) => CString::new("Transaction completed successfully")
            .unwrap()
            .into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn programmable_transaction_allow_sponser(
    sender_address: *const c_char,
    recipient_address: *const c_char,
    amount: u64,
    sponser_address: *const c_char,
) -> *const c_char {
    let sender = unsafe { CStr::from_ptr(sender_address).to_string_lossy().to_string() };
    let sponser = unsafe {
        CStr::from_ptr(sponser_address)
            .to_string_lossy()
            .to_string()
    };
    let recipient = unsafe {
        CStr::from_ptr(recipient_address)
            .to_string_lossy()
            .to_string()
    };

    // Here we run the async block synchronously for simplicity
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            ProgrammableTransactionAllowSponser(&sender, &recipient, amount, &sponser).await
        });

    match result {
        Ok(_) => CString::new("Transaction completed successfully")
            .unwrap()
            .into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn request_tokens_from_faucet_(address_str: *const c_char) -> *const c_char {
    let address = unsafe { CStr::from_ptr(address_str).to_string_lossy().to_string() };

    // Run the async function synchronously inside the Rust environment
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { request_tokens_from_faucet(&address).await });

    match result {
        Ok(_) => CString::new("Request successful").unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}
