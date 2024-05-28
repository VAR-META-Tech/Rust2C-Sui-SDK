use std::ffi::{c_char, CString};
use sui_sdk::SuiClientBuilder;
use super::SuiClientSingleton;
use anyhow::{Ok, Result};
use futures::{future, stream::StreamExt};
use sui_sdk::{
    error::SuiRpcResult,
    types::base_types::{ObjectID, SuiAddress},
};

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