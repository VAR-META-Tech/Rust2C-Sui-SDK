use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sui_sdk::{SuiClient, SuiClientBuilder};
use tokio::sync::Mutex;

#[derive(Clone)]
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

    async fn initialize(&self, environment: SuiEnvironment) -> Result<()> {
        let mut env_guard = self.environment.lock().await;
        if env_guard.is_some() {
            return Err(anyhow!("Environment already initialized"));
        }
        *env_guard = Some(environment);
        Ok(())
    }

    async fn get_or_init(&self) -> Result<SuiClient> {
        let mut env_guard = self.environment.lock().await;
        let environment = if let Some(env) = &*env_guard {
            env.clone()
        } else {
            let default_env = SuiEnvironment::Devnet;
            *env_guard = Some(default_env.clone());
            default_env
        };

        let mut client_guard = self.client.lock().await;
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
