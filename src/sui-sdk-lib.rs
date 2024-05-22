use std::ffi::{c_char, c_int, CString};

use sui_client::{_api_version, _available_rpc_methods, _available_subscriptions, _check_api_version};
use sui_sdk::{SuiClient, SuiClientBuilder};
// Import the necessary crates
use anyhow::Result;
use tokio::runtime; // Using Tokio as the async runtime

mod coin_read_api;
mod sui_client;
mod utils;
use coin_read_api::_coin_read_api;
mod event_api;
use event_api::_event_api;
mod connect_sui_api;
use connect_sui_api::{connect_devnet, connect_localnet, connect_testnet};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

struct SuiClientSingleton {
    client: Mutex<Option<SuiClient>>,
}

impl SuiClientSingleton {
    fn instance() -> &'static SuiClientSingleton {
        static INSTANCE: OnceCell<SuiClientSingleton> = OnceCell::new();
        INSTANCE.get_or_init(|| SuiClientSingleton {
            client: Mutex::new(None),
        })
    }

    async fn get_or_init(&self) -> Result<SuiClient, sui_sdk::error::Error> {
        let mut client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            Ok(client.clone())
        } else {
            let client = SuiClientBuilder::default().build_testnet().await?;
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

// Struct to hold C-compatible string array
#[repr(C)]
pub struct CStringArray {
    data: *const *const c_char,
    len: c_int,
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
