use std::str::FromStr;

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use anyhow::{Ok, Result};
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
//mod utils;
use super::SuiClientSingleton;
use futures::{future, stream::StreamExt};
use sui_sdk::types::balance::Supply;
use sui_json_rpc_types::{Balance, Coin, Page};
//use utils::setup_for_read;

// This example uses the coin read api to showcase the available
// functions to retrieve coin related information for a specific address.
// The example will use the active address in the wallet (if it exists or create one if it doesn't)
// check if it has coins and request coins from the faucet if there aren't any.
// If there is no wallet, it will create a wallet and two addresses, set one address as active,
// and add 1 SUI to the active address.
// By default, the example will use the Sui testnet network (fullnode.testnet.sui.io:443).

pub async fn get_coins() -> Result<Page<Coin, ObjectID>> {
    let (sui, active_address) = super::utils::setup_for_read().await?;
    let coins = sui
        .coin_read_api()
        .get_coins(active_address, None, None, None)
        .await?;
    println!(" *** Coins ***");
    println!("{:?}", coins);
    
    for (index, coin) in coins.data.iter().enumerate() {
        println!("Coin {}:", index);
        println!("  Coin Type: {}", coin.coin_type);
       
        let value = coin.coin_object_id.to_string();
        println!("  Coin Object ID: {:?}", value);
        println!("  Version: {}", coin.version.value());
        println!("  Digest: {:?}", coin.digest);
        println!("  Balance: {}", coin.balance);
        println!("  Previous Transaction: {:?}", coin.previous_transaction);
    }
    println!(" *** Coins ***\n");
    Ok(coins)
}

pub async fn get_total_supply() -> Result<Supply> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
 // Total Supply
    let total_supply: Supply = sui
    .coin_read_api()
    .get_total_supply("0x2::sui::SUI".to_string())
    .await?;
    println!(" *** Total Supply *** ");
    println!("{:?}", total_supply);
    println!(" *** Total Supply ***\n ");
    Ok(total_supply)
}
pub async fn get_balance() -> Result<Balance> {
    let (sui, active_address) = super::utils::setup_for_read().await?;
      // Balance
    // Returns the balance for the specified coin type for this address,
    // or if None is passed, it will use Coin<SUI> as the coin type
    let balance = sui
        .coin_read_api()
        .get_balance(active_address, None)
        .await?;
    println!(" *** Balance ");
    println!("Balance: {:?}", balance);
    Ok(balance)
}
pub async fn get_all_balances() -> Result<Vec<Balance>> {
    let (sui, active_address) = super::utils::setup_for_read().await?;
      // Balance
   // Total balance
    // Returns the balance for each coin owned by this address
    let total_balance = sui.coin_read_api().get_all_balances(active_address).await?;
    println!(" *** Total Balance *** ");
    println!("Total Balance: {:?}", total_balance);
    println!(" *** Total Balance ***\n ");
    Ok(total_balance)
}

pub async fn _coin_read_api() -> Result<()> {
    let (sui, active_address) = super::utils::setup_for_read().await?;

    // ************ COIN READ API ************ //

    // Get coins for this address. Coins can be filtered by `coin_type`
    // (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC) or
    // use `None` for the default `Coin<SUI>` which is represented as
    // "0x2::sui::SUI"
    let coin_type = Some("0x2::sui::SUI".to_string());
    let coins = sui
        .coin_read_api()
        .get_coins(active_address, coin_type.clone(), None, Some(5)) // get the first five coins
        .await?;
    println!(" *** Coins ***");
    println!("{:?}", coins);
    println!(" *** Coins ***\n");

    // Get all coins
    // This function works very similar to the get_coins function, except it does not take
    // a coin_type filter argument and it returns all coin types associated with this address
    let all_coins = sui
        .coin_read_api()
        .get_all_coins(active_address, None, Some(5)) // get the first five coins
        .await?;
    println!(" *** All coins ***");
    println!("{:?}", all_coins);
    println!(" *** All coins ***\n");

    // Get coins as a stream
    // Similar to the previous functions, except it returns the coins as a stream.
    let coins_stream = sui.coin_read_api().get_coins_stream(active_address, None);

    println!(" *** Coins Stream ***");
    coins_stream
        .for_each(|coin| {
            println!("{:?}", coin);
            future::ready(())
        })
        .await;
    println!(" *** Coins Stream ***\n");

    // Select coins based on the provided coin type (SUI in this example). Use `None` for the default Sui coin
    let select_coins = sui
        .coin_read_api()
        .select_coins(active_address, coin_type, 1, vec![])
        .await?;

    println!(" *** Select Coins ***");
    println!("{:?}", select_coins);
    println!(" *** Select Coins ***\n");

    // Balance
    // Returns the balance for the specified coin type for this address,
    // or if None is passed, it will use Coin<SUI> as the coin type
    let balance = sui
        .coin_read_api()
        .get_balance(active_address, None)
        .await?;

    // Total balance
    // Returns the balance for each coin owned by this address
    let total_balance = sui.coin_read_api().get_all_balances(active_address).await?;
    println!(" *** Balance + Total Balance *** ");
    println!("Balance: {:?}", balance);
    println!("Total Balance: {:?}", total_balance);
    println!(" *** Balance + Total Balance ***\n ");

    // Return the coin metadata for the Coin<SUI>
    let coin_metadata = sui
        .coin_read_api()
        .get_coin_metadata("0x2::sui::SUI".to_string())
        .await?;

    println!(" *** Coin Metadata *** ");
    println!("{:?}", coin_metadata);
    println!(" *** Coin Metadata ***\n ");

    // Total Supply
    let total_supply = sui
        .coin_read_api()
        .get_total_supply("0x2::sui::SUI".to_string())
        .await?;
    println!(" *** Total Supply *** ");
    println!("{:?}", total_supply);
    println!(" *** Total Supply ***\n ");

    // ************ END OF COIN READ API ************ //
    Ok(())
}
