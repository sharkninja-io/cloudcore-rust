use std::os::raw::{c_char, c_uint};
use ffi_utilities::{MantleOptionString, RustCBridge};

#[repr(C)]
#[derive(Debug)]
pub struct IoTDevice {
    id: *const u32,
    product_name: *const c_char,
    model: *const c_char,
    dsn: *const c_char,
    oem_model: *const c_char,
    sw_version: *const c_char,
    template_id: *const c_uint,
    mac: *const c_char,
    lan_ip: *const c_char,
    connected_at: *const c_char,
    lan_enabled: *const bool,
    has_properties: *const bool,
    connection_status: *const c_char,
    lat: *const c_char,
    lng: *const c_char,
    device_type: *const c_char,
}

impl RustCBridge<cloudcore::devices::IoTDevice> for IoTDevice {
    fn new_c_object(rust_device: &cloudcore::devices::IoTDevice) -> Self {
        Self {
            id: match rust_device.id() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null()
            },
            product_name: MantleOptionString(rust_device.product_name().to_owned()).to_ptr(),
            model: MantleOptionString(rust_device.model().to_owned()).to_ptr(),
            dsn: MantleOptionString(rust_device.dsn().to_owned()).to_ptr(),
            oem_model: MantleOptionString(rust_device.oem_model().to_owned()).to_ptr(),
            sw_version: MantleOptionString(rust_device.sw_version().to_owned()).to_ptr(),
            template_id: match rust_device.template_id() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null()
            },
            mac: MantleOptionString(rust_device.mac().to_owned()).to_ptr(),
            lan_ip: MantleOptionString(rust_device.lan_ip().to_owned()).to_ptr(),
            connected_at: MantleOptionString(rust_device.connected_at().to_owned()).to_ptr(),
            lan_enabled: match rust_device.lan_enabled() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null()
            },
            has_properties: match rust_device.has_properties() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null()
            },
            connection_status: MantleOptionString(rust_device.connection_status().to_owned()).to_ptr(),
            lat: MantleOptionString(rust_device.lat().to_owned()).to_ptr(),
            lng: MantleOptionString(rust_device.lng().to_owned()).to_ptr(),
            device_type: MantleOptionString(rust_device.device_type().to_owned()).to_ptr(),
        }
    }
}
