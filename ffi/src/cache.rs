use cloudcore::{CloudCore};
use std::os::raw::{c_char};
use ffi_utilities::{convert_to_using_mantle_error, MantleStringPointer, RuntimeFFI};
use mantle_utilities::MantleError;
use serde_json::Value;
use cloudcore::cache::{CacheDataValue, CacheInteract};

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_set_value(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char,
    cache_value: *mut CacheDataValue,
    callback: fn(result: Result<(), Box<MantleError>>)
) {
    let cache_value = *Box::from_raw(cache_value);
    let cloudcore = &mut *ptr_cloudcore;
    let path = MantleStringPointer(path).to_string();
    let key = MantleStringPointer(key).to_string();
    let value: Value =  match cache_value {
        CacheDataValue::StringValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::IntegerValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::DoubleValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::BooleanValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::ObjectValue(it) => it,
        CacheDataValue::NullValue => Value::Null
    };
 
    let closure = move || {
        cloudcore.cache.set_value(path, key, value)
    };
    RuntimeFFI::exec_sync(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_set_value_sync(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char,
    cache_value: *mut CacheDataValue,
) -> Result<(), Box<MantleError>> {
    let cache_value = *Box::from_raw(cache_value);
    let cloudcore = &mut *ptr_cloudcore;
    let path = MantleStringPointer(path).to_string();
    let key = MantleStringPointer(key).to_string();
    let value: Value =  match cache_value {
        CacheDataValue::StringValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::IntegerValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::DoubleValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::BooleanValue(it) => serde_json::to_value(it).unwrap_or(Value::Null),
        CacheDataValue::ObjectValue(it) => it,
        CacheDataValue::NullValue => Value::Null
    };

    convert_to_using_mantle_error(cloudcore.cache.set_value(path, key, value))
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_value(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char,
    callback: fn(result: Result<CacheDataValue, Box<MantleError>>)
) {
    let path = MantleStringPointer(path).to_string();
    let key = MantleStringPointer(key).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = move || {
        cloudcore.cache.get_value(path, key)
    };
    RuntimeFFI::exec_sync(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_value_sync(
    ptr_cloudcore: *mut CloudCore,
    path: *const c_char,
    key: *const c_char
) -> Result<CacheDataValue, Box<MantleError>> {
    let path = MantleStringPointer(path).to_string();
    let key = MantleStringPointer(key).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    convert_to_using_mantle_error(cloudcore.cache.get_value(path, key))
}