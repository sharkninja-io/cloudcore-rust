mod value;

use crate::cache::value::CachedDataValue;
use std::os::raw::{c_char};
use ffi_utilities::{CRustBridge, MantleResult};
use ios_utilities::{CallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;
use cloudcore::cache::CacheDataValue;
use cloudcore::CloudCore;

lazy_static! {
    static ref SET_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref GET_CB_STRUCT: Mutex<CallbackStruct<CachedDataValue>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_set_value(
        ptr_cloudcore: *mut CloudCore,
        path: *const c_char,
        key: *const c_char,
        cache_value: *mut CacheDataValue,
        callback: fn(result: Result<(), Box<MantleError>>)
    );
    fn cloudcore_get_value(
        ptr_cloudcore: *mut CloudCore,
        path: *const c_char,
        key: *const c_char,
        callback: fn(result: Result<CacheDataValue, Box<MantleError>>)
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_set_value(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char,
    ptr_value: *mut CachedDataValue,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    let cache_value = CachedDataValue::new_rust_object(ptr_value).unwrap();
    SET_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    let boxed_value = Box::into_raw(Box::new(cache_value));
    cloudcore_set_value(ptr_cloudcore, path, key, boxed_value, |result| {
        SET_CB_STRUCT.lock().unwrap().run(result);
    });
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_value(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char,
    callback: fn(result: MantleResult<CachedDataValue>, callback_id: u64),
    callback_id: u64,
) {
    GET_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_get_value(ptr_cloudcore, path, key, |result| {
        GET_CB_STRUCT.lock().unwrap().run(result);
    });
}
