
use sui_sdk::SuiClientBuilder;
// Import the necessary crates
use anyhow::Result;
use tokio::runtime; // Using Tokio as the async runtime

mod utils;
mod coin_read_api;
use coin_read_api::_coin_read_api;
mod event_api;
use event_api::_event_api;
mod sui_client;
use sui_client::_sui_clients;
async fn async_connects_testnet() -> Result<()> {
    // Simulate some async operations
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());
    Ok(())
}

async fn async_connects_devnet() -> Result<()> {
    // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());
    // Return Ok or Err
    Ok(())
}


#[no_mangle]

pub extern "C" fn async_connects_testnet_c() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match async_connects_testnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]

pub extern "C" fn async_connects_devnet_c() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match async_connects_devnet().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}


async fn _connect_sui() -> Result<()> {
    // Simulate some async operations
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());

    // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());
    // Return Ok or Err
    Ok(())
}

#[no_mangle]
pub extern "C" fn connect_sui() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();

    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _connect_sui().await {
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
pub extern "C" fn sui_clients() -> i32 {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();

    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        match _sui_clients().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

