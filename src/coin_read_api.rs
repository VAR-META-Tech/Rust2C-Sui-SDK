use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use std::{ffi::c_char, str::FromStr};
use std::result::Result::Ok;

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use anyhow::{Result};
use sui_sdk::{types::base_types::{ObjectID, SuiAddress}, SuiClientBuilder};
use tokio::runtime;
use crate::balance::{self, get_all_balances, get_balance, get_coins, get_total_supply};
use futures::{future, stream::StreamExt};
use sui_sdk::types::balance::Supply;
use sui_json_rpc_types::{Balance, Coin, Page};

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

//public functions for ffi
// C-compatible Balance struct
#[repr(C)]
pub struct CBalance {
    coin_type: *const c_char,
    coin_object_count: usize,
    total_balance: [u64; 2],
}

// C-compatible vector of balances
#[repr(C)]
pub struct CBalanceArray {
    balances: *const CBalance,
    length: usize,
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

#[no_mangle]
pub extern "C" fn get_total_supply_sync() -> u64 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(get_total_supply());
    match result {
        Ok(supply) => supply.value,
        Err(_) => 0, // Return 0 in case of error
    }
}

#[no_mangle]
pub extern "C" fn get_balance_sync(address: *const c_char) -> CBalance {
    // This is a placeholder function that simulates fetching a Balance
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let balance = runtime
        .block_on(get_balance(address_str))
        .unwrap_or_else(|_| Balance {
            coin_type: "".to_string(),
            coin_object_count: 0,
            total_balance: 0,
            locked_balance: HashMap::new(),
        });
    let total_balance_bytes = balance.total_balance.to_le_bytes();
    CBalance {
        coin_type: CString::new(balance.coin_type.clone()).unwrap().into_raw(),
        coin_object_count: balance.coin_object_count,
        total_balance: [
            u64::from_le_bytes(total_balance_bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(total_balance_bytes[8..16].try_into().unwrap()),
        ],
    }
}

#[no_mangle]
pub extern "C" fn free_balance(balance: CBalance) {
    if !balance.coin_type.is_null() {
        unsafe {
            drop(CString::from_raw(balance.coin_type as *mut c_char));
        }
    }
}

/// Wrapper for the Balance struct to implement methods
pub struct BalanceWrapper(Balance);

impl BalanceWrapper {
    fn to_c_balance(&self) -> CBalance {
        let total_balance_bytes = self.0.total_balance.to_le_bytes();
        CBalance {
            coin_type: CString::new(self.0.coin_type.clone()).unwrap().into_raw(),
            coin_object_count: self.0.coin_object_count,
            total_balance: [
                u64::from_le_bytes(total_balance_bytes[0..8].try_into().unwrap()),
                u64::from_le_bytes(total_balance_bytes[8..16].try_into().unwrap()),
            ],
        }
    }
}
// Function to convert a vector of Balances to a CBalanceArray
fn to_c_balance_array(balances: Vec<Balance>) -> CBalanceArray {
    let c_balances: Vec<CBalance> = balances
        .iter()
        .map(|b| BalanceWrapper(b.clone()).to_c_balance())
        .collect();
    let length = c_balances.len();
    let balances_ptr = c_balances.as_ptr();
    std::mem::forget(c_balances); // Prevent Rust from freeing the memory
    CBalanceArray {
        balances: balances_ptr,
        length,
    }
}

// Function to free a CBalanceArray
#[no_mangle]
pub extern "C" fn free_balance_array(balance_array: CBalanceArray) {
    if !balance_array.balances.is_null() {
        unsafe {
            let balances_slice = std::slice::from_raw_parts_mut(
                balance_array.balances as *mut CBalance,
                balance_array.length,
            );
            for balance in balances_slice.iter() {
                if !balance.coin_type.is_null() {
                    drop(CString::from_raw(balance.coin_type as *mut c_char));
                }
            }
            drop(Vec::from_raw_parts(
                balance_array.balances as *mut CBalance,
                balance_array.length,
                balance_array.length,
            ));
        }
    }
}

#[no_mangle]
pub extern "C" fn get_all_balances_sync(address: *const c_char) -> CBalanceArray {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let balances = runtime
        .block_on(get_all_balances(address_str))
        .unwrap_or_else(|_| Vec::new());
    to_c_balance_array(balances)
}

#[no_mangle]
pub extern "C" fn get_balances(address: *const c_char) -> CBalanceArray {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let balances = runtime
        .block_on(balance::_get_all_balances(address_str))
        .unwrap_or_else(|_| Vec::new());
    to_c_balance_array(balances)
}

// C-compatible structures
#[repr(C)]
pub struct CCoin {
    coin_type: *mut c_char,
    coin_object_id: *mut c_char, // Changed to array of 32 bytes
    version: u64,
    digest: *mut c_char,
    balance: u64,
    previous_transaction: *mut c_char,
}

#[repr(C)]
pub struct CCoinArray {
    coins: *const CCoin,
    length: usize,
}

// Function to convert a vector of WrappedCoins to a CCoinArray
fn to_c_coin_array(coins: Vec<WrappedCoin>) -> CCoinArray {
    let c_coins: Vec<CCoin> = coins.iter().map(|c| c.to_c_coin()).collect();
    let length = c_coins.len();
    let coins_ptr = c_coins.as_ptr();
    std::mem::forget(c_coins); // Prevent Rust from freeing the memory
    CCoinArray {
        coins: coins_ptr,
        length,
    }
}

// Function to free a CCoinArray
#[no_mangle]
pub extern "C" fn free_coin_array(coin_array: CCoinArray) {
    if !coin_array.coins.is_null() {
        unsafe {
            drop(Vec::from_raw_parts(
                coin_array.coins as *mut CCoin,
                coin_array.length,
                coin_array.length,
            ));
        }
    }
}

// Synchronous wrapper to call the async get_coins function
#[no_mangle]
pub extern "C" fn get_coins_sync(address: *const c_char) -> CCoinArray {
    let c_str = unsafe {
        assert!(!address.is_null());
        CStr::from_ptr(address)
    };
    let address_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let coins = runtime
        .block_on(get_coins(address_str))
        .unwrap_or_else(|_| Page {
            data: Vec::new(),
            next_cursor: None,
            has_next_page: false,
        });

    let wrapped_coins: Vec<WrappedCoin> = coins
        .data
        .into_iter()
        .map(|inner| WrappedCoin { inner })
        .collect();
    to_c_coin_array(wrapped_coins)
}

// Wrapper struct for Coin
pub struct WrappedCoin {
    pub inner: sui_json_rpc_types::Coin,
}

fn string_to_c_char(s: Option<String>) -> *mut c_char {
    match s {
        Some(str) => CString::new(str).unwrap().into_raw(),
        None => ptr::null_mut(),
    }
}
impl WrappedCoin {
    pub fn to_c_coin(&self) -> CCoin {
        CCoin {
            coin_type: CString::new(self.inner.coin_type.clone())
                .unwrap()
                .into_raw(),
            coin_object_id: string_to_c_char(Some(self.inner.coin_object_id.to_string())),
            version: self.inner.version.value(),
            digest: string_to_c_char(Some(self.inner.digest.base58_encode())),
            balance: self.inner.balance,
            previous_transaction: string_to_c_char(Some(
                self.inner.previous_transaction.base58_encode(),
            )),
        }
    }
}