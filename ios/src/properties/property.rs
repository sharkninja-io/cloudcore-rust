use std::os::raw::{c_char, c_uint};
use ffi_utilities::{MantleOptionString, MantleString, RustCBridge};
use crate::properties::value::IoTPropertyValue;
use cloudcore::properties::property;

#[repr(C)]
#[derive(Debug)]
pub struct IoTProperty {
    r#type: *const c_char,
    name: *const c_char,
    base_type: *const c_char,
    read_only: bool,
    direction: *const c_char,
    scope: *const c_char,
    data_updated_at: *const c_char,
    key: *const c_uint,
    device_key: *const c_uint,
    product_name: *const c_char,
    track_only_changes: bool,
    display_name: *const c_char,
    host_sw_version: bool,
    time_series: bool,
    derived: bool,
    app_type: *const c_char,
    recipe: *const c_char,
    value: *const IoTPropertyValue,
    ack_enabled: bool,
    ack_status: *const c_char,
    ack_message: *const c_char,
    acked_at: *const c_char,
}

impl RustCBridge<property::IoTProperty> for IoTProperty {
    fn new_c_object(rust_property: &property::IoTProperty) -> Self {
        Self {
            r#type: MantleString(rust_property.r#type().to_owned()).to_ptr(),
            name: MantleString(rust_property.name().to_owned()).to_ptr(),
            base_type: MantleString(rust_property.base_type().to_owned()).to_ptr(),
            read_only: rust_property.read_only().to_owned(),
            direction: MantleString(rust_property.direction().to_owned()).to_ptr(),
            scope: MantleString(rust_property.scope().to_owned()).to_ptr(),
            data_updated_at: MantleOptionString(rust_property.data_updated_at().to_owned()).to_ptr(),
            key: match rust_property.key() {
                Some(key) => Box::into_raw(Box::new(key)),
                None => std::ptr::null()
            },
            device_key: match rust_property.key() {
                Some(key) => Box::into_raw(Box::new(key)),
                None => std::ptr::null()
            },
            product_name: MantleString(rust_property.product_name().to_owned()).to_ptr(),
            track_only_changes: rust_property.track_only_changes().to_owned(),
            display_name: MantleString(rust_property.display_name().to_owned()).to_ptr(),
            host_sw_version: rust_property.host_sw_version().to_owned(),
            time_series: rust_property.time_series().to_owned(),
            derived: rust_property.derived().to_owned(),
            app_type: MantleOptionString(rust_property.app_type()).to_ptr(),
            recipe: MantleOptionString(rust_property.recipe()).to_ptr(),
            value: match rust_property.value() {
                Some(val) => Box::into_raw(Box::new(IoTPropertyValue::new_c_object(val))),
                None => std::ptr::null()
            },
            ack_enabled: rust_property.ack_enabled().to_owned(),
            ack_status: MantleOptionString(rust_property.ack_status()).to_ptr(),
            ack_message: MantleOptionString(rust_property.ack_message()).to_ptr(),
            acked_at: MantleOptionString(rust_property.acked_at()).to_ptr()
        }
    }
}