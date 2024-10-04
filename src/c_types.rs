use std::ffi::{c_char, c_int, c_uchar, c_uint, CStr, CString};

use move_core_types::u256::{self, U256};
use sui_json_rpc_types::{SuiData, SuiObjectData};

#[repr(C)]
pub struct CSuiObjectData {
    pub object_id: *mut c_char,
    pub version: u64,
    pub digest: *mut c_char,
    pub type_: *mut c_char,
    pub owner: *mut c_char,
    pub previous_transaction: *mut c_char,
    pub storage_rebate: u64,
    pub display: *mut c_char,
    pub content: *mut c_char,
    pub bcs: *mut c_char,
}

impl CSuiObjectData {
    pub fn from(data: SuiObjectData) -> Self {
        let content = data
            .content
            .unwrap()
            .try_as_move()
            .unwrap()
            .fields
            .clone()
            .to_json_value()
            .to_string();
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
            content: CString::new(content).unwrap().into_raw(),
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

#[repr(C)]
pub struct CSuiObjectDataArray {
    pub data: *mut CSuiObjectData,
    pub len: usize,
}

// Struct to hold C-compatible string array
#[repr(C)]
pub struct CStringArray {
    pub data: *const *const c_char,
    pub len: c_int,
}

#[repr(C)]
pub struct CU8Array {
    pub data: *const c_uchar,
    pub len: c_uint,
    pub error: *const c_char,
}

pub struct CPure {
    pub data: Vec<u8>
}

// Struct to hold the result, either CStringArray or error message
#[repr(C)]
pub struct ResultCStringArray {
    pub strings: CStringArray,
    pub error: *const c_char,
}

// Function to free the C-compatible string array
#[no_mangle]
pub extern "C" fn free_strings(array: CStringArray) {
    unsafe {
        for i in 0..array.len {
            let c_str_ptr = *array.data.add(i as usize);
            if !c_str_ptr.is_null() {
                drop(CString::from_raw(c_str_ptr as *mut c_char));
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn free_sui_object_data_list(array: CSuiObjectDataArray) {
    if array.data.is_null() {
        return;
    }
    unsafe {
        let boxed_slice = Box::from_raw(std::slice::from_raw_parts_mut(array.data, array.len));
        for obj in boxed_slice.iter() {
            if !obj.object_id.is_null() {
                CString::from_raw(obj.object_id);
            }
            if !obj.digest.is_null() {
                CString::from_raw(obj.digest);
            }
            if !obj.type_.is_null() {
                CString::from_raw(obj.type_);
            }
            if !obj.owner.is_null() {
                CString::from_raw(obj.owner);
            }
            if !obj.previous_transaction.is_null() {
                CString::from_raw(obj.previous_transaction);
            }
            if !obj.display.is_null() {
                CString::from_raw(obj.display);
            }
            if !obj.content.is_null() {
                CString::from_raw(obj.content);
            }
            if !obj.bcs.is_null() {
                CString::from_raw(obj.bcs);
            }
        }
    }
}

// Function to free the error string
#[no_mangle]
pub extern "C" fn free_error_string(error: *const c_char) {
    if !error.is_null() {
        unsafe {
            CString::from_raw(error as *mut c_char);
        }
    }
}

#[no_mangle]
pub extern "C" fn bsc_basic(type_: *const c_char, data: *const c_char) -> *mut CPure {
    let type_str = unsafe { CStr::from_ptr(type_).to_string_lossy() };
    let data_str = unsafe { CStr::from_ptr(data).to_string_lossy() };
    let result = match type_str.as_ref() {
        "u8" => {
            let value = data_str.parse::<u8>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "u64" => {
            let value = data_str.parse::<u64>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "u128" => {
            let value = data_str.parse::<u128>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "u256" => {
            let value = data_str.parse::<U256>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "i8" => {
            let value = data_str.parse::<i8>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "i64" => {
            let value = data_str.parse::<i64>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "i128" => {
            let value = data_str.parse::<i128>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "f32" => {
            let value = data_str.parse::<f32>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "f64" => {
            let value = data_str.parse::<f64>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "bool" => {
            let value = data_str.parse::<bool>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "uleb128" => {
            let value = data_str.parse::<u128>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "string" => {
            let value = data_str.to_string();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        "address" => {
            let value = data_str.parse::<move_core_types::account_address::AccountAddress>().unwrap();
            let bytes = bcs::to_bytes(&value).unwrap();
            CPure { data: bytes}
        }
        _ => CPure {
            data: Vec::new(),
        },
    };
    Box::into_raw(Box::new(result))
}

#[repr(C)]
#[derive(Clone)]
pub struct CByteVector {
    pub data: *mut u8,
    pub size: usize,
    pub capacity: usize,
}

impl From<Vec<u8>> for CByteVector {
    fn from(vec: Vec<u8>) -> Self {
        let mut boxed_vec = vec.into_boxed_slice();
        let data = boxed_vec.as_mut_ptr();
        let size = boxed_vec.len();
        let capacity = boxed_vec.len();
        std::mem::forget(boxed_vec);
        CByteVector { data, size, capacity }
    }
}