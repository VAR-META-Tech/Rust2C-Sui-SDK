use crate::c_types::{self, CStringArray, CU8Array};
use crate::sui_client::SuiClientSingleton;
use core::slice;
use shared_crypto::intent::{Intent, IntentMessage};
use std::ffi::{c_uint, CStr, CString};
use std::result::Result::Ok;
use std::str::FromStr;
use std::{ffi::c_char, path::PathBuf};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::TransactionData,
    },
};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::PublicKey;
use sui_types::crypto::Signature;
use sui_types::multisig::{MultiSig, MultiSigPublicKey, WeightUnit};
use sui_types::signature::GenericSignature;
use sui_types::transaction::{Argument, Command, Transaction};
use tokio::runtime;

pub fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}
pub async fn get_or_create_multisig_public_key(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<MultiSigPublicKey, anyhow::Error> {
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let mut pk_map: Vec<(PublicKey, WeightUnit)> = vec![];
    for (index, address) in addresses.iter().enumerate() {
        pk_map.push((
            keystore
                .get_key(&SuiAddress::from_str(address)?)?
                .public()
                .clone(),
            weights[index],
        ))
    }
    Ok(MultiSigPublicKey::insecure_new(pk_map, threshold))
}

pub async fn get_or_create_multisig_public_key_serialize(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<Vec<u8>, anyhow::Error> {
    Ok(bcs::to_bytes(
        &get_or_create_multisig_public_key(addresses, weights, threshold).await?,
    )?)
}

pub async fn get_or_create_multisig_address(
    addresses: Vec<&str>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<String, anyhow::Error> {
    Ok(
        SuiAddress::from(&get_or_create_multisig_public_key(addresses, weights, threshold).await?)
            .to_string(),
    )
}

pub async fn create_sui_transaction(
    multisig_addr: &str,
    recipient_address: &str,
    amount: u64,
) -> Result<TransactionData, anyhow::Error> {
    let multisig_addr = SuiAddress::from_str(multisig_addr)?;
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let recipient = SuiAddress::from_str(recipient_address)?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    // 2) split coin
    // the amount we want in the new coin, 1000 MIST
    let split_coint_amount = ptb.pure(amount)?; // note that we need to specify the u64 type
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coint_amount],
    ));
    // 3) transfer the new coin to a different address
    let argument_address = ptb.pure(recipient)?;
    ptb.command(Command::TransferObjects(
        vec![Argument::Result(0)],
        argument_address,
    ));
    let coins = sui_client
        .coin_read_api()
        .get_coins(multisig_addr, None, None, None) // get the first five coins
        .await?;
    let selected_gas_coins: Vec<_> = coins.data.iter().map(|coin| coin.object_ref()).collect();
    let builder = ptb.finish();
    let gas_budget = 5_000_000;
    let gas_price = sui_client.read_api().get_reference_gas_price().await?;
    Ok(TransactionData::new_programmable(
        multisig_addr,
        selected_gas_coins,
        builder,
        gas_budget,
        gas_price,
    ))
}

pub async fn _sign_and_execute_transaction(
    tx_data: Vec<u8>,
    signers_addresses: Vec<&str>,
    multisig_pk: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let sui_client = SuiClientSingleton::instance().get_or_init().await?;
    let keystore_path = default_keystore_path();
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let tx_data: TransactionData = bcs::from_bytes(&tx_data)?;
    let multisig_pk: MultiSigPublicKey = bcs::from_bytes(&multisig_pk)?;
    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    let mut signatures = Vec::with_capacity(signers_addresses.len());
    for address in signers_addresses {
        signatures.push(
            GenericSignature::from(Signature::new_secure(
                &intent_msg,
                keystore.get_key(&SuiAddress::from_str(address)?)?,
            ))
            .to_compressed()
            .unwrap(),
        );
    }

    let multisig =
        GenericSignature::MultiSig(MultiSig::insecure_new(signatures, 0b011, multisig_pk));

    let tx = Transaction::from_generic_sig_data(tx_data, vec![multisig]);
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(tx, SuiTransactionBlockResponseOptions::full_content(), None)
        .await?;
    println!(
        "Transaction executed. Transaction digest: {}",
        transaction_response.digest.base58_encode()
    );
    if !transaction_response.status_ok().unwrap_or(false) {
        return Err(anyhow::Error::msg("Transaction failed"));
    }
    Ok(())
}

//public function for ffi

#[repr(C)]
pub struct CMultiSig {
    address: *const c_char,
    bytes: c_types::CU8Array,
    error: *const c_char,
}

#[no_mangle]
pub extern "C" fn free_multisig(multisig: CMultiSig) {
    unsafe {
        if !multisig.address.is_null() {
            let _ = CString::from_raw(multisig.address as *mut c_char);
        }
        if !multisig.error.is_null() {
            let _ = CString::from_raw(multisig.error as *mut c_char);
        }
        if !multisig.bytes.data.is_null() {
            let _ = Box::from_raw(slice::from_raw_parts_mut(
                multisig.bytes.data as *mut u8,
                multisig.bytes.len as usize,
            ));
        }
    }
}

#[no_mangle]
pub extern "C" fn get_or_create_multisig(
    addresses: c_types::CStringArray,
    weights: c_types::CU8Array,
    threshold: u16,
) -> CMultiSig {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        let addresses: Vec<&str> = unsafe {
            (0..addresses.len)
                .map(|i| {
                    let c_str = CStr::from_ptr(*addresses.data.add(i as usize));
                    c_str.to_str().expect("Invalid UTF-8")
                })
                .collect()
        };
        let weights: Vec<u8> =
            unsafe { slice::from_raw_parts(weights.data, weights.len as usize).to_vec() };

        match get_or_create_multisig_public_key(addresses, weights, threshold).await {
            Ok(multisig_pk) => {
                let bytes = bcs::to_bytes(&multisig_pk).unwrap();
                println!("Vec<u8> in Rust: {:?}", bytes);

                let boxed_bytes = bytes.into_boxed_slice();
                let data_ptr = boxed_bytes.as_ptr();
                let len = boxed_bytes.len() as c_uint;

                // Leak the boxed slice to keep it alive
                std::mem::forget(boxed_bytes);
                CMultiSig {
                    bytes: c_types::CU8Array {
                        data: data_ptr,
                        len: len,
                        error: std::ptr::null(),
                    },
                    address: CString::new(SuiAddress::from(&multisig_pk).to_string())
                        .unwrap()
                        .into_raw(),
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap().into_raw();
                CMultiSig {
                    bytes: c_types::CU8Array {
                        data: std::ptr::null(),
                        len: 0,
                        error: std::ptr::null(),
                    },
                    address: std::ptr::null(),
                    error: error_message,
                }
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn sign_and_execute_transaction_miltisig(
    multisig: CU8Array,
    tx: CU8Array,
    addresses: CStringArray,
) -> *const c_char {
    // Create a new runtime. This step might vary based on the async runtime you are using.
    let rt = runtime::Runtime::new().unwrap();
    // Block on the async function and translate the Result to a C-friendly format.
    rt.block_on(async {
        let addresses: Vec<&str> = unsafe {
            (0..addresses.len)
                .map(|i| {
                    let c_str = CStr::from_ptr(*addresses.data.add(i as usize));
                    c_str.to_str().expect("Invalid UTF-8")
                })
                .collect()
        };
        let tx: Vec<u8> = unsafe { slice::from_raw_parts(tx.data, tx.len as usize).to_vec() };
        let multisig: Vec<u8> =
            unsafe { slice::from_raw_parts(multisig.data, multisig.len as usize).to_vec() };
        match _sign_and_execute_transaction(tx, addresses, multisig).await {
            Ok(()) => {
                let success_message = CString::new("Sign and execute transaction success").unwrap();
                success_message.into_raw() // Return the raw pointer to the C string
            }
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                error_message.into_raw() // Return the raw pointer to the C string
            }
        }
    })
}
