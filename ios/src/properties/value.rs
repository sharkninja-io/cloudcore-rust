use std::os::raw::c_char;
use ffi_utilities::{CRustBridge, MantleString, MantleStringPointer, RustCBridge};
use cloudcore::properties::value;

#[repr(C)]
#[derive(Debug)]
pub enum IoTPropertyValue {
    Int(i32),
    Str(*const c_char),
    Bool(bool),
}

impl RustCBridge<value::IoTPropertyValue> for IoTPropertyValue {
    fn new_c_object(rust_value: &value::IoTPropertyValue) -> Self {
        match rust_value {
            value::IoTPropertyValue::Int(value) => IoTPropertyValue::Int(*value),
            value::IoTPropertyValue::Str(value) => IoTPropertyValue::Str(MantleString(value.to_string()).to_ptr()),
            value::IoTPropertyValue::Bool(value) => IoTPropertyValue::Bool(*value)
        }
    }
}

impl CRustBridge<value::IoTPropertyValue> for IoTPropertyValue {
    unsafe fn new_rust_object(c_object_ptr: *const Self) -> Option<value::IoTPropertyValue> {
        if c_object_ptr.is_null() {
            Option::None
        } else {
            match c_object_ptr.as_ref() {
                None => Option::None,
                Some(obj_ref) => Some(
                    match obj_ref {
                        IoTPropertyValue::Int(value) => value::IoTPropertyValue::Int(*value),
                        IoTPropertyValue::Str(value) => value::IoTPropertyValue::Str(MantleStringPointer(*value).to_string()),
                        IoTPropertyValue::Bool(value) => value::IoTPropertyValue::Bool(*value)
                    }
                ),
            }
        }
    }
}