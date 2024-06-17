use once_cell::sync::OnceCell;
use std::sync::Mutex;
use sui_sdk::{error::Error, SuiClient, SuiClientBuilder};

pub enum SuiEnvironment {
    Testnet,
    Devnet,
}

struct SuiClientSingleton {
    client: Mutex<Option<SuiClient>>,
    environment: Mutex<Option<SuiEnvironment>>,
}

impl SuiClientSingleton {
    fn instance() -> &'static SuiClientSingleton {
        static INSTANCE: OnceCell<SuiClientSingleton> = OnceCell::new();
        INSTANCE.get_or_init(|| SuiClientSingleton {
            client: Mutex::new(None),
            environment: Mutex::new(None),
        })
    }

    async fn initialize(&self, environment: SuiEnvironment) -> Result<(), Error> {
        let mut env_guard = self.environment.lock().unwrap();
        if env_guard.is_some() {
            return Err(Error::DataError(
                "Environment already initialized".to_string(),
            ));
        }
        *env_guard = Some(environment);
        Ok(())
    }

    async fn get_or_init(&self) -> Result<SuiClient, Error> {
        let env_guard = self.environment.lock().unwrap();
        let environment = match &*env_guard {
            Some(env) => env.clone(),
            None => return Err(Error::DataError("Environment not initialized".to_string())),
        };

        let mut client_guard = self.client.lock().unwrap();
        if let Some(client) = &*client_guard {
            Ok(client.clone())
        } else {
            let client = match environment {
                SuiEnvironment::Testnet => SuiClientBuilder::default().build_testnet().await?,
                SuiEnvironment::Devnet => SuiClientBuilder::default().build_devnet().await?,
            };
            *client_guard = Some(client.clone());
            Ok(client)
        }
    }
}

#[tokio::main]
async fn main() {
    let sui_client_singleton = SuiClientSingleton::instance();

    // Initialize environment only once
    match sui_client_singleton
        .initialize(SuiEnvironment::Testnet)
        .await
    {
        Ok(()) => println!("Environment initialized to Testnet."),
        Err(e) => eprintln!("Failed to initialize environment: {:?}", e),
    }

    // Try to re-initialize with a different environment (should fail)
    match sui_client_singleton
        .initialize(SuiEnvironment::Devnet)
        .await
    {
        Ok(()) => println!("Environment re-initialized to Devnet."),
        Err(e) => eprintln!("Failed to re-initialize environment: {:?}", e),
    }

    // Get or initialize the SuiClient
    match sui_client_singleton.get_or_init().await {
        Ok(client) => println!("SuiClient initialized successfully!"),
        Err(e) => eprintln!("Failed to initialize SuiClient: {:?}", e),
    }
}
