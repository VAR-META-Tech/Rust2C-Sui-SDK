use anyhow::Result;
use sui_sdk::SuiClientBuilder;

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
