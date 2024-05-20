// Import the necessary crates
use tokio::runtime; // Using Tokio as the async runtime
mod coin_read_api;
mod connect_sui;
mod event_api;
mod sui_clients;
mod utils;

#[no_mangle]
pub extern "C" fn connect_sui() -> i32 {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match connect_sui::_connect_sui().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn coin_read_api() -> i32 {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match coin_read_api::coin_read_api().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn event_api() -> i32 {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match event_api::event_api().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}

#[no_mangle]
pub extern "C" fn sui_clients() -> i32 {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match sui_clients::sui_clients().await {
            Ok(_) => 0,  // Return 0 to indicate success.
            Err(_) => 1, // Return 1 or other error codes to indicate an error.
        }
    })
}
