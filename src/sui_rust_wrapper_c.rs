use sui_sdk::SuiClientBuilder;
// Import the necessary crates
use anyhow::Result;
use tokio::runtime; // Using Tokio as the async runtime

mod utils;
mod coin_read_api;
use coin_read_api::_coin_read_api;
mod event_api;
use event_api::_event_api;
mod connect_sui_api;
use connect_sui_api::{connect_localnet,connect_devnet,connect_testnet};

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


