use super::SuiClientSingleton;
use std::str::FromStr;

use anyhow::{Ok, Result};
use futures::{future, stream::StreamExt};
use sui_json_rpc_types::{Balance, Coin, Page};
use sui_sdk::types::balance::Supply;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::SuiClientBuilder;

pub async fn _get_coins() -> Result<Page<Coin, ObjectID>> {
    let sui_client = SuiClientBuilder::default().build_testnet().await?;
    let address =
        SuiAddress::from_str("0x0cc4b15265e0a342a2822377258e3750ecea621172e580395674790b33844a6b")
            .unwrap();

    let coins = sui_client
        .coin_read_api()
        .get_coins(address, None, None, None)
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

pub async fn _get_balance() -> Result<Balance> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    // let sui_client = SuiClientBuilder::default().build_testnet().await?;
    let address =
        SuiAddress::from_str("0x0cc4b15265e0a342a2822377258e3750ecea621172e580395674790b33844a6b")
            .unwrap();
    let balance = sui_client
        .coin_read_api()
        .get_balance(address, None)
        .await?;
    println!(" *** Balance ");
    println!("Balance: {:?}", balance);
    Ok(balance)
}
pub async fn _get_all_balances(address: &str) -> Result<Vec<Balance>> {
    println!(" ***  _get_all_balances *** ");

    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    println!(" ***  sui_client done *** ");

    // let sui_client = SuiClientBuilder::default().build_testnet().await?;
    let address = SuiAddress::from_str(address).unwrap();
    // Balance
    // Total balance
    // Returns the balance for each coin owned by this address
    let total_balance = sui_client.coin_read_api().get_all_balances(address).await?;
    println!(" *** Total Balance *** ");
    println!("Total Balance: {:?}", total_balance);
    println!(" *** Total Balance ***\n ");
    Ok(total_balance)
}
pub async fn get_coins(address: &str) -> Result<Page<Coin, ObjectID>> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    let active_address: SuiAddress = SuiAddress::from_str(address)?;
    println!("Read Coin From address:{}", active_address.to_string());

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
pub async fn get_balance(address: &str) -> Result<Balance> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    let active_address: SuiAddress = SuiAddress::from_str(address)?;
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
pub async fn get_all_balances(address: &str) -> Result<Vec<Balance>> {
    let sui = SuiClientSingleton::instance().get_or_init().await?;
    let active_address: SuiAddress = SuiAddress::from_str(address)?;
      // Balance
   // Total balance
    // Returns the balance for each coin owned by this address
    let total_balance = sui.coin_read_api().get_all_balances(active_address).await?;
    println!(" *** Total Balance *** ");
    println!("Total Balance: {:?}", total_balance);
    println!(" *** Total Balance ***\n ");
    Ok(total_balance)
}
