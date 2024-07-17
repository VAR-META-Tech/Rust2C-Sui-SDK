mod utils;
use anyhow::anyhow;
use fastcrypto::encoding::Base58;
use move_core_types::language_storage::StructTag;
use rand::seq::index;
use shared_crypto::intent::Intent;
use std::{
    ffi::{c_char, CStr, CString},
    str::FromStr,
};
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::{
    SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectRef, SuiObjectResponseQuery,
};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ProgrammableMoveCall, Transaction, TransactionData,
        },
        Identifier,
    },
    SuiClientBuilder,
};
use sui_types::{
    base_types::{ObjectRef, SequenceNumber, SuiAddress},
    digests::ObjectDigest,
    object::{Object, ObjectRead},
    transaction::ObjectArg,
};
use utils::setup_for_write;

// This example shows how to use programmable transactions to chain multiple
// commands into one transaction, and specifically how to call a function from a move package
// These are the following steps:
// 1) finds a coin from the active address that has Sui,
// 2) creates a PTB and adds an input to it,
// 3) adds a move call to the PTB,
// 4) signs the transaction,
// 5) executes it.
// For some of these actions it prints some output.
// Finally, at the end of the program it prints the number of coins for the
// Sui address that received the coin.
// If you run this program several times, you should see the number of coins
// for the recipient address increases.

#[repr(C)]
pub struct CSuiObjectData {
    object_id: *mut c_char,
    version: u64,
    digest: *mut c_char,
    type_: *mut c_char,
    owner: *mut c_char,
    previous_transaction: *mut c_char,
    storage_rebate: u64,
    display: *mut c_char,
    content: *mut c_char,
    bcs: *mut c_char,
}

impl CSuiObjectData {
    fn from(data: SuiObjectData) -> Self {
        CSuiObjectData {
            object_id: CString::new(data.object_id.to_string()).unwrap().into_raw(),
            version: data.version.value(),
            digest: CString::new(data.digest.to_string()).unwrap().into_raw(),
            type_: match data.type_ {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            owner: match data.owner {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            previous_transaction: match data.previous_transaction {
                Some(obj) => CString::new(obj.to_string()).unwrap().into_raw(),
                None => CString::new("None").unwrap().into_raw(),
            },
            storage_rebate: data.storage_rebate.unwrap_or_default(),
            display: CString::new(format!("{:?}", data.display))
                .unwrap()
                .into_raw(),
            content: CString::new(format!("{:?}", data.content))
                .unwrap()
                .into_raw(),
            bcs: CString::new(format!("{:?}", data.bcs)).unwrap().into_raw(),
        }
    }
    pub fn show(&self) {
        unsafe {
            println!("object_id: {}", self.c_str_to_string(self.object_id));
            println!("version: {}", self.version);
            println!("digest: {}", self.c_str_to_string(self.digest));
            println!("type_: {}", self.c_str_to_string(self.type_));
            println!("owner: {}", self.c_str_to_string(self.owner));
            println!(
                "previous_transaction: {}",
                self.c_str_to_string(self.previous_transaction)
            );
            println!("storage_rebate: {}", self.storage_rebate);
            println!("display: {}", self.c_str_to_string(self.display));
            println!("content: {}", self.c_str_to_string(self.content));
            println!("bcs: {}", self.c_str_to_string(self.bcs));
        }
    }

    unsafe fn c_str_to_string(&self, c_str: *mut c_char) -> String {
        if c_str.is_null() {
            String::from("null")
        } else {
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui_client = SuiClientBuilder::default().build_devnet().await?;
    let active_address: SuiAddress =
        SuiAddress::from_str("0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd")?;

    let query = Some(SuiObjectResponseQuery {
        filter: Some(SuiObjectDataFilter::StructType(StructTag::from_str(
            "0xd1efbd86210322b550a8d6017ad5113fda2bd4f486593096f83e7b9ce3cbd002::nft::NFT",
        )?)),
        options: Some(SuiObjectDataOptions::new().with_type()),
    });

    let owned_objects = sui_client
        .read_api()
        .get_owned_objects(active_address, query, None, None)
        .await?
        .data;

    let data: Vec<SuiObjectData> = owned_objects
        .into_iter()
        .filter_map(|owned_objects| owned_objects.data)
        .collect();
    // println!(" *** Owned Objects data ***");
    // println!("{:?}", data);
    // println!(" *** Owned Objects data ***\n");
    // println!("{:?}", data[1].object_id.to_string());

    let c_data: Vec<CSuiObjectData> = data.into_iter().map(CSuiObjectData::from).collect();
    println!(" *** Owned Objects cdata ***");
    for (_index, data) in c_data.into_iter().enumerate() {
        data.show()
    }
    println!(" *** Owned Objects cdata ***\n");
    Ok(())
}
