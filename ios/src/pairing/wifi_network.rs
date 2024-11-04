use log::debug;
use std::os::raw::{c_char, c_int, c_uint};
use ffi_utilities::{MantleOptionString, MantleStringPointer, CRustBridge, RustCBridge};

#[repr(C)]
#[derive(Debug)]
pub struct WifiNetwork {
    bars: *const c_uint,
    bssid: *const c_char,
    chan: *const c_uint,
    security: *const c_char,
    signal: *const c_int,
    ssid: *const c_char,
    r#type: *const c_char,
    password: *const c_char,
}

impl RustCBridge<cloudcore::pairing::wifi_network::WifiNetwork> for WifiNetwork {
    fn new_c_object(rust_object: &cloudcore::pairing::wifi_network::WifiNetwork) -> Self {
        Self {
            bars: match rust_object.bars() {
                Some(bars) => Box::into_raw(Box::new(bars)),
                None => std::ptr::null(),
            },
            bssid: MantleOptionString(rust_object.bssid()).to_ptr(),
            chan: match rust_object.chan() {
                Some(chan) => Box::into_raw(Box::new(chan)),
                None => std::ptr::null(),
            },
            security: MantleOptionString(rust_object.security()).to_ptr(),
            signal: match rust_object.signal() {
                Some(signal) => Box::into_raw(Box::new(signal)),
                None => std::ptr::null(),
            },
            ssid: MantleOptionString(rust_object.ssid()).to_ptr(),
            r#type: MantleOptionString(rust_object.r#type()).to_ptr(),
            password: MantleOptionString(rust_object.password()).to_ptr(),
        }
    }
}

impl CRustBridge<cloudcore::pairing::wifi_network::WifiNetwork> for WifiNetwork {
    unsafe fn new_rust_object(
        c_object_ptr: *const Self,
    ) -> Option<cloudcore::pairing::wifi_network::WifiNetwork> {
        if c_object_ptr.is_null() {
            debug!("wifi pointer was null");
            Option::None
        } else {
            let obj_ref = & *c_object_ptr;
            let network = cloudcore::pairing::wifi_network::WifiNetwork::new(
                if obj_ref.bars.is_null() {
                    None
                } else {
                    let bars = Box::from_raw(obj_ref.bars as *mut u32);
                    Some(*bars)
                },
                MantleStringPointer(obj_ref.bssid).to_option_string(),
                if obj_ref.chan.is_null() {
                    None
                } else {
                    let chan = Box::from_raw(obj_ref.chan as *mut u32);
                    Some(*chan)
                },
                MantleStringPointer(obj_ref.security).to_option_string(),
                if obj_ref.signal.is_null() {
                    None
                } else {
                    let signal = Box::from_raw(obj_ref.signal as *mut i32);
                    Some(*signal)
                },
                MantleStringPointer(obj_ref.ssid).to_option_string(),
                MantleStringPointer(obj_ref.r#type).to_option_string(),
                MantleStringPointer(obj_ref.password).to_option_string(),
            );
            Some(network)
        }
    }
}
