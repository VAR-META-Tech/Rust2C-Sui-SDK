use sui_sdk::SuiClientBuilder;
// Import the necessary crates
use anyhow::Result;
use tokio::runtime; // Using Tokio as the async runtime
mod utils;
use futures::{future, stream::StreamExt};
use utils::setup_for_read;

use sui_sdk::rpc_types::EventFilter;
use utils::{setup_for_write, split_coin_digest};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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

async fn _coin_read_api() -> Result<()> {
    let (sui, active_address) = setup_for_read().await?;

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

async fn _event_api() -> Result<()> {
    let (sui, active_address, _second_address) = setup_for_write().await?;

    println!(" *** Get events *** ");
    // for demonstration purposes, we set to make a transaction
    let digest = split_coin_digest(&sui, &active_address).await?;
    let events = sui.event_api().get_events(digest).await?;
    println!("{:?}", events);
    println!(" *** Get events ***\n ");

    let descending = true;
    let query_events = sui
        .event_api()
        .query_events(EventFilter::All(vec![]), None, Some(5), descending) // query first 5 events in descending order
        .await?;
    println!(" *** Query events *** ");
    println!("{:?}", query_events);
    println!(" *** Query events ***\n ");

    let ws = SuiClientBuilder::default()
        .ws_url("wss://rpc.testnet.sui.io:443")
        .build("https://fullnode.testnet.sui.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .event_api()
        .subscribe_event(EventFilter::All(vec![]))
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
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

async fn _sui_clients() -> Result<()> {
    let sui = SuiClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Sui local network version: {}", sui.api_version());

    // local Sui network, like the above one but using the dedicated function
    let sui_local = SuiClientBuilder::default().build_localnet().await?;
    println!("Sui local network version: {}", sui_local.api_version());

    // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());

    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());

    println!("{:?}", sui_local.available_rpc_methods());
    println!("{:?}", sui_local.available_subscriptions());

    Ok(())
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
