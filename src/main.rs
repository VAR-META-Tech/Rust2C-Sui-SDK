use once_cell::sync::OnceCell;
use sui_sdk::error::Error;
use sui_sdk::{SuiClient, SuiClientBuilder};
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

    async fn get_or_init(&self) -> Result<SuiClient, Error> {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
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
