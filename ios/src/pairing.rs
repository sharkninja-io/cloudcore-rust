mod wifi_network;

use crate::pairing::wifi_network::WifiNetwork as iOSWifiNetwork;
use cloudcore::{CloudCore, WifiPairing, WifiPairingState, WifiNetwork};
use std::os::raw::c_char;
use ffi_utilities::{MantleResult, RustCBridge, MantleList, CRustBridge};
use ios_utilities::CallbackHolder;
use lazy_static::lazy_static;
use std::sync::Mutex;
use log::error;
use mantle_utilities::MantleError;

lazy_static! {
    static ref STATE_CB: Mutex<CallbackHolder<WifiPairingState>> = Mutex::new(CallbackHolder::new());
    static ref NETWORKS_CB_STRUCT: Mutex<CallbackHolder<*const MantleList<iOSWifiNetwork>>> = Mutex::new(CallbackHolder::new());
    static ref RESULT_CB_STRUCT: Mutex<CallbackHolder<MantleResult<*const c_char>>> = Mutex::new(CallbackHolder::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_create_pairing_manager(
        ptr_cloudcore: *const CloudCore,
        get_state_callback: fn(
            state: WifiPairingState,
        ),
        wifi_networks_callback: fn(
            wifi_networks: Vec<WifiNetwork>,
        ),
        result_callback: fn(
            result: Result<String, Box<MantleError>>
        ),
    ) -> *mut WifiPairing;
    fn cloudcore_start_pairing(
        ptr_wifi_manager: *mut WifiPairing,
        ip_address: *const c_char
    );
    fn cloudcore_continue_pairing(
        ptr_wifi_manager: *mut WifiPairing
    );
    fn cloudcore_done_pairing(
        ptr: *mut WifiPairing
    );
    fn cloudcore_handle_selected_network(
        ptr_wifi_manager: *mut WifiPairing,
        selected_network: *mut WifiNetwork
    );
    fn cloudcore_write_to_pairing_log(
        ptr_cloudcore: *const CloudCore,
        c_content: *const c_char,
    ) -> Result<(), Box<MantleError>>;
    fn cloudcore_get_pairing_log(
        ptr_cloudcore: *const CloudCore,
    ) -> Result<String, Box<MantleError>>;
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_create_pairing_manager(
    ptr_cloudcore: *const CloudCore,
    get_state_callback: fn(
        state: WifiPairingState,
    ),
    get_wifi_networks_callback: fn(
        wifi_networks: *const MantleList<iOSWifiNetwork>,
    ),
    result_callback: fn(result: MantleResult<*const c_char>,),
) -> *mut WifiPairing {
    STATE_CB.lock().unwrap().update(get_state_callback);
    NETWORKS_CB_STRUCT.lock().unwrap().update(get_wifi_networks_callback);
    RESULT_CB_STRUCT.lock().unwrap().update(result_callback);
    cloudcore_create_pairing_manager(
        ptr_cloudcore,
        handle_state,
        handle_wifi_networks,
        handle_result
    )
}

fn handle_state(state: WifiPairingState) {
    STATE_CB.lock().unwrap().run_for_repeat(state);
}

fn handle_wifi_networks(networks: Vec<WifiNetwork>) {
    let wifi_networks = MantleList::<iOSWifiNetwork>::boxed_list(networks);
    NETWORKS_CB_STRUCT.lock().unwrap().run_for_repeat(wifi_networks);
}

fn handle_result(result: Result<String, Box<MantleError>>) {
    let result = MantleResult::new_c_object(&result);
    RESULT_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_start_pairing(
    ptr_wifi_manager: *mut WifiPairing,
    ip_address: *const c_char
) {
    cloudcore_start_pairing(ptr_wifi_manager, ip_address);
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_continue_pairing(ptr_wifi_manager: *mut WifiPairing) {
    cloudcore_continue_pairing(ptr_wifi_manager);
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_done_pairing(ptr_wifi_manager: *mut WifiPairing) {
    if ptr_wifi_manager.is_null() {
        error!("in ios wifi pairing object pointer is null");
        return;
    }
    STATE_CB.lock().unwrap().reset();
    NETWORKS_CB_STRUCT.lock().unwrap().reset();
    cloudcore_done_pairing(ptr_wifi_manager);
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_handle_selected_network(ptr_wifi_manager: *mut WifiPairing, selected_network: *const iOSWifiNetwork) {
     if let Some(network) = iOSWifiNetwork::new_rust_object(selected_network) {
         let boxed_network = Box::into_raw(Box::new(network));
         cloudcore_handle_selected_network(ptr_wifi_manager, boxed_network);
     } else {
         error!("passed in iOS network object could not be converted to Rust");
     }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_write_to_pairing_log(
    ptr_cloudcore: *const CloudCore,
    c_content: *const c_char,
) -> MantleResult<()> {
    MantleResult::new_c_object(&cloudcore_write_to_pairing_log(ptr_cloudcore, c_content))
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_pairing_log(
    ptr_cloudcore: *const CloudCore,
) -> MantleResult<*const c_char> {
    MantleResult::new_c_object(&cloudcore_get_pairing_log(ptr_cloudcore))
}