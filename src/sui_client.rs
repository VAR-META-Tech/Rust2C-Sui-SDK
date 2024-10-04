use anyhow::{Result, anyhow};
use futures::{future, stream::StreamExt};
use once_cell::sync::OnceCell;
use tokio::runtime;
use std::ffi::{c_char, c_int, CString};
use tokio::sync::Mutex;
use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_sdk::{
    error::SuiRpcResult,
    types::base_types::{ObjectID, SuiAddress},
};

use crate::c_types::{CStringArray, ResultCStringArray};

/// Returns a list of RPC methods supported by the node the client is connected to.
pub async fn _available_rpc_methods() -> Result<Vec<String>, anyhow::Error> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    Ok(sui.available_rpc_methods().clone())
}

/// Returns a list of streaming/subscription APIs supported by the node the client is connected to.
pub async fn _available_subscriptions() -> Result<Vec<String>, anyhow::Error> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    Ok(sui.available_subscriptions().clone())
}

/// Returns the API version information as a string.
///
/// The format of this string is `<major>.<minor>.<patch>`, e.g., `1.6.0`,
/// and it is retrieved from the OpenRPC specification via the discover service method.
pub async fn _api_version() -> Result<*const c_char> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    let stf = sui.api_version();
    Ok(CString::new(stf).unwrap().into_raw())
}

/// Verifies if the API version matches the server version and returns an error if they do not match.
pub async fn _check_api_version() -> Result<()> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    sui.check_api_version()?;
    Ok(())
}

pub async fn connect_localnet() -> Result<()> {
    // Sui localnet -- http://127.0.0.1:9000
    let sui_localnet = SuiClientBuilder::default().build_localnet().await?;
    println!("Sui local network version: {}", sui_localnet.api_version());
    Ok(())
}

pub async fn connect_devnet() -> Result<()> {
    // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());
    // Return Ok or Err
    Ok(())
}

pub async fn connect_testnet() -> Result<()> {
    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());
    Ok(())
}

//public functions for FFI
#[derive(Clone)]
pub enum SuiEnvironment {
    Testnet,
    Devnet,
    Mainnet,
}

pub struct SuiClientSingleton {
    client: Mutex<Option<SuiClient>>,
    environment: Mutex<Option<SuiEnvironment>>,
}

impl SuiClientSingleton {
    pub fn instance() -> &'static SuiClientSingleton {
        static INSTANCE: OnceCell<SuiClientSingleton> = OnceCell::new();
        INSTANCE.get_or_init(|| SuiClientSingleton {
            client: Mutex::new(None),
            environment: Mutex::new(None),
        })
    }

    async fn initialize(&self, environment: SuiEnvironment) -> Result<()> {
        let mut env_guard = self.environment.lock().await;
        if env_guard.is_some() {
            return Err(anyhow::anyhow!("Environment already initialized"));
        }
        *env_guard = Some(environment);
        Ok(())
    }

    pub async fn get_or_init(&self) -> Result<SuiClient> {
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
                SuiEnvironment::Mainnet => SuiClientBuilder::default().build("https://fullnode.mainnet.sui.io:443").await?,
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
            std::result::Result::Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

pub async fn _build_mainnet() -> Result<()> {
    let sui_client_singleton = SuiClientSingleton::instance();

    // Initialize environment only once
    match sui_client_singleton
        .initialize(SuiEnvironment::Mainnet)
        .await
    {
        Ok(()) => println!("Environment initialized to Mainnet."),
        Err(e) => eprintln!("Failed to initialize environment: {:?}", e),
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn build_mainnet() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _build_mainnet().await {
            std::result::Result::Ok(_) => 0,  // Return 0 to indicate success.
            std::result::Result::Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}


pub async fn _build_testnet() -> Result<()> {
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
pub extern "C" fn build_testnet() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _build_testnet().await {
            std::result::Result::Ok(_) => 0,  // Return 0 to indicate success.
            std::result::Result::Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

pub async fn _build_devnet() -> Result<()> {
    let sui_client_singleton = SuiClientSingleton::instance();

    // Initialize environment only once
    match sui_client_singleton
        .initialize(SuiEnvironment::Devnet)
        .await
    {
        std::result::Result::Ok(()) => println!("Environment initialized to Devnet."),
        Err(e) => eprintln!("Failed to initialize environment: {:?}", e),
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn build_devnet() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _build_devnet().await {
            std::result::Result::Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
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