use std::os::raw::c_char;
use ffi_utilities::{MantleOptionString, MantleString, RustCBridge};
use cloudcore::properties::datapoint;
use crate::properties::value::IoTPropertyValue;

#[repr(C)]
#[derive(Debug)]
pub struct IoTDatapoint {
    value: *const IoTPropertyValue,
    user_uuid: *const c_char,
    updated_at: *const c_char,
    created_at: *const c_char,
    echo: *const bool,
}

impl RustCBridge<datapoint::IoTDatapoint> for IoTDatapoint {
    fn new_c_object(rust_datapoint: &datapoint::IoTDatapoint) -> Self {
        Self {
            value: Box::into_raw(Box::new(IoTPropertyValue::new_c_object(rust_datapoint.value()))),
            user_uuid: MantleOptionString(rust_datapoint.metadata().user_uuid()).to_ptr(),
            updated_at: MantleOptionString(rust_datapoint.updated_at().to_owned()).to_ptr(),
            created_at: MantleOptionString(rust_datapoint.created_at().to_owned()).to_ptr(),
            echo: match rust_datapoint.echo() {
                Some(echo) => Box::into_raw(Box::new(echo)),
                None => std::ptr::null()
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IoTDatapointFile {
    updated_at: *const c_char,
    created_at: *const c_char,
    echo: bool,
    closed: bool,
    generated_at: *const c_char,
    generated_from: *const c_char,
    value: *const c_char,
    created_at_from_device: *const c_char,
    file: *const c_char,
    local_file: *const c_char,
}

impl RustCBridge<datapoint::IoTDatapointFile> for IoTDatapointFile {
    fn new_c_object(rust_file_datapoint: &datapoint::IoTDatapointFile) -> Self {
        Self {
            updated_at: MantleString(rust_file_datapoint.updated_at().to_owned()).to_ptr(),
            created_at: MantleString(rust_file_datapoint.created_at().to_owned()).to_ptr(),
            echo: rust_file_datapoint.echo().to_owned(),
            closed: rust_file_datapoint.closed().to_owned(),
            generated_at: MantleOptionString(rust_file_datapoint.generated_at()).to_ptr(),
            generated_from: MantleOptionString(rust_file_datapoint.generated_from()).to_ptr(),
            value: MantleString(rust_file_datapoint.value().to_owned()).to_ptr(),
            created_at_from_device: MantleOptionString(rust_file_datapoint.created_at_from_device()).to_ptr(),
            file: MantleString(rust_file_datapoint.file().to_owned()).to_ptr(),
            local_file: MantleOptionString(rust_file_datapoint.local_file()).to_ptr(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IoTDatapointMessage {
    user_uuid: *const c_char,
    updated_at: *const c_char,
    created_at: *const c_char,
    echo: *const bool,
    local_file: *const c_char
}

impl RustCBridge<datapoint::IoTDatapointMessage> for IoTDatapointMessage {
    fn new_c_object(rust_datapoint: &datapoint::IoTDatapointMessage) -> Self {
        Self {
            user_uuid: MantleOptionString(rust_datapoint.metadata.user_uuid()).to_ptr(),
            updated_at: MantleOptionString(rust_datapoint.updated_at.as_ref().to_owned()).to_ptr(),
            created_at: MantleOptionString(rust_datapoint.created_at.as_ref().to_owned()).to_ptr(),
            echo: match rust_datapoint.echo {
                Some(echo) => Box::into_raw(Box::new(echo)),
                None => std::ptr::null()
            },
            local_file: MantleString(rust_datapoint.local_file.to_owned()).to_ptr(),
        }
    }
}