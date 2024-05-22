use std::ffi::{c_char, CString};

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
