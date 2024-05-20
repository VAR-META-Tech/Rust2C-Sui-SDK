// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod coin_read_api;
mod utils;

// This example uses the coin read api to showcase the available
// functions to retrieve coin related information for a specific address.
// The example will use the active address in the wallet (if it exists or create one if it doesn't)
// check if it has coins and request coins from the faucet if there aren't any.
// If there is no wallet, it will create a wallet and two addresses, set one address as active,
// and add 1 SUI to the active address.
// By default, the example will use the Sui testnet network (fullnode.testnet.sui.io:443).

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = coin_read_api::_coin_read_api().await;
    Ok(())
}
