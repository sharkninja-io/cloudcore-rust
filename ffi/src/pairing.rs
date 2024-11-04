use cloudcore::{CloudCore, wifi_manager, WifiPairing, WifiPairingState};
use std::os::raw::c_char;
use ffi_utilities::{convert_to_using_mantle_error, MantleStringPointer};
use log::error;
use mantle_utilities::MantleError;
use cloudcore::pairing::wifi_network::WifiNetwork;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_create_pairing_manager(
    ptr_cloudcore: *const CloudCore,
    get_state_callback: fn(
        state: WifiPairingState,
    ),
    wifi_networks_callback: fn(
        wifi_networks: Vec<WifiNetwork>,
    ),
    done_callback: fn(result: Result<String, Box<MantleError>>),
) -> *mut WifiPairing {
    let cloudcore = &*ptr_cloudcore;
    let wifi_manager = cloudcore.create_pairing_manager(
        Box::new(move |state| {
            get_state_callback(
                state,
            )
        }),
        Box::new(move |networks| {
            wifi_networks_callback(
                networks,
            )
        }),
        Box::new(move |result| {
            done_callback(convert_to_using_mantle_error(result));
        }),
    );
    Box::into_raw(Box::new(wifi_manager))
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_start_pairing(
    ptr_wifi_manager: *mut WifiPairing,
    ip_address: *const c_char
) {
    if ptr_wifi_manager.is_null() {
        error!("wifi pairing object pointer is null");
        return;
    }
    let wifi_manager = &mut *ptr_wifi_manager;
    let ip = MantleStringPointer(ip_address).to_string();
    wifi_manager.start(ip)
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_continue_pairing(
    ptr_wifi_manager: *mut WifiPairing
) {
    if ptr_wifi_manager.is_null() {
        error!("wifi pairing object pointer is null");
        return;
    }
    let wifi_manager = &mut *ptr_wifi_manager;
    wifi_manager.continue_pairing()
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_done_pairing(
    ptr_wifi_manager: *mut WifiPairing
) {
    if ptr_wifi_manager.is_null() {
        error!("wifi pairing object pointer is null");
        return;
    }
    let wifi_manager = *Box::from_raw(ptr_wifi_manager);
    wifi_manager.done_pairing()
}

// Keep this around just in case
#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn cloudcore_pairing_state(
    ptr_wifi_manager: *mut WifiPairing
) -> *const WifiPairingState {
    if ptr_wifi_manager.is_null() {
        error!("wifi pairing object pointer is null");
        return std::ptr::null();
    }
    let wifi_manager = *Box::from_raw(ptr_wifi_manager);
    Box::into_raw(Box::new(wifi_manager.state().clone()))
}


#[no_mangle]
extern "C" fn cloudcore_handle_selected_network(ptr_wifi_manager: *mut WifiPairing, selected_network: *mut WifiNetwork) {
    if ptr_wifi_manager.is_null() {
        error!("wifi pairing object pointer is null");
        return;
    }
    let manager = unsafe { &mut *ptr_wifi_manager };
    let wifi_network = unsafe { *Box::from_raw(selected_network) };
    wifi_manager::handle_wifi_network(manager, wifi_network)
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_write_to_pairing_log(
    ptr_cloudcore: *const CloudCore,
    c_content: *const c_char,
) -> Result<(), Box<MantleError>> {
    let cloudcore = &*ptr_cloudcore;
    let content = MantleStringPointer(c_content).to_string();
    convert_to_using_mantle_error(cloudcore.write_to_pairing_log(content))
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_pairing_log(
    ptr_cloudcore: *const CloudCore,
) -> Result<String, Box<MantleError>> {
    let cloudcore = &*ptr_cloudcore;
    convert_to_using_mantle_error(cloudcore.get_pairing_log())
}