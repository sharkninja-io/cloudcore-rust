use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_int};
use ffi_utilities::{CRustBridge, MantleString, MantleStringPointer, RustCBridge};
use serde_json::Value;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum CachedDataValue {
    CDV_STRING_VALUE(*const c_char),
    CDV_INTEGER_VALUE(c_int),
    CDV_DOUBLE_VALUE(c_double),
    CDV_BOOLEAN_VALUE(bool),
    CDV_OBJECT_VALUE(*const c_char),
    CDV_NULL_VALUE(*const c_void)
}

impl RustCBridge<cloudcore::cache::CacheDataValue> for CachedDataValue {
    fn new_c_object(rust_value: &cloudcore::cache::CacheDataValue) -> Self {
        match rust_value {
            cloudcore::cache::CacheDataValue::StringValue(it) => {
                let ptr = MantleString(it.to_owned()).to_ptr();
                CachedDataValue::CDV_STRING_VALUE(ptr)
            }
            cloudcore::cache::CacheDataValue::IntegerValue(it) => {
                CachedDataValue::CDV_INTEGER_VALUE(*it)
            }
            cloudcore::cache:: CacheDataValue::DoubleValue(it) => {
                CachedDataValue::CDV_DOUBLE_VALUE(*it)
            }
            cloudcore::cache::CacheDataValue::BooleanValue(it) => {
                CachedDataValue::CDV_BOOLEAN_VALUE(it.to_owned())
            }
            cloudcore::cache::CacheDataValue::ObjectValue(it) => {
                let str = serde_json::to_string(it).unwrap_or("{}".to_string());
                let ptr = MantleString(str).to_ptr();
                CachedDataValue::CDV_OBJECT_VALUE(ptr)
            }
            cloudcore::cache::CacheDataValue::NullValue => {
                CachedDataValue::CDV_NULL_VALUE(std::ptr::null())
            }
        }
    }
}

impl CRustBridge<cloudcore::cache::CacheDataValue> for CachedDataValue {
    unsafe fn new_rust_object(c_object_ptr: *const Self) -> Option<cloudcore::cache::CacheDataValue> {
        if c_object_ptr.is_null() {
            None
        } else {
            match c_object_ptr.as_ref() {
                None => None,
                Some(obj_ref) => Some(
                    match obj_ref {
                        CachedDataValue::CDV_STRING_VALUE(it) => {
                            let str =  MantleStringPointer(it.to_owned()).to_string();
                            cloudcore::cache::CacheDataValue::StringValue(str)
                        }
                        CachedDataValue::CDV_INTEGER_VALUE(it) => {
                            cloudcore::cache::CacheDataValue::IntegerValue(*it)
                        }
                        CachedDataValue::CDV_DOUBLE_VALUE(it) => {
                            cloudcore::cache::CacheDataValue::DoubleValue(*it)
                        }
                        CachedDataValue::CDV_BOOLEAN_VALUE(it) => {
                            cloudcore::cache::CacheDataValue::BooleanValue(*it)
                        }
                        CachedDataValue::CDV_OBJECT_VALUE(it) => {
                            let string = MantleStringPointer(it.to_owned()).to_string();
                            let value = serde_json::from_str(string.as_str()).unwrap_or(Value::Null);
                            cloudcore::cache::CacheDataValue::ObjectValue(value)
                        }
                        CachedDataValue::CDV_NULL_VALUE(_) => {
                            cloudcore::cache::CacheDataValue::NullValue
                        }
                    }
                ),
            }
        }
    }
}