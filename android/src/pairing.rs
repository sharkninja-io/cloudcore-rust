mod wifi_network;
mod wifi_state;

use android_utilities::{AndroidList, AndroidResult, AndroidStringResult, CallbackStruct, invoke_result_callback, JavaClass, JObjectRustBridge};
use android_utilities::java_class_names::CLASSREFSMAP;
use android_utilities::java_signatures::VOID_SIG;
use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jobject::{invoke_callback, MantleJObject};
use android_utilities::jni_exts::jstring::MantleJString;
use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jlong, jobject};
use lazy_static::lazy_static;
use std::sync::Mutex;
use log::{debug, error};
use mantle_utilities::MantleError;
use cloudcore::{CloudCore, WifiNetwork, WifiPairing, WifiPairingState};
use crate::cloudcore_ffi_api::CLOUDCORE_API;
use crate::pairing::wifi_network::{JavaWifiNetwork, WIFI_NETWORK_SIG};
use crate::pairing::wifi_state::{JavaWifiPairingState, WIFI_PAIRING_STATE_SIG};

lazy_static! {
    static ref STATE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref NETWORKS_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref RESULT_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_createPairingManager(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_state_callback: JObject,
    j_wifi_networks_callback: JObject,
    j_result_callback: JObject,
) -> *const WifiPairing {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();

    STATE_CB_STRUCT.lock().unwrap().update(env, j_state_callback);
    NETWORKS_CB_STRUCT.lock().unwrap().update(env, j_wifi_networks_callback);
    RESULT_CB_STRUCT.lock().unwrap().update(env, j_result_callback);
    CLOUDCORE_API.cloudcore_create_pairing_manager(
        cloudcore,
        handle_state,
        handle_networks,
        handle_result
    )
}

fn handle_callback<Rust>(cb_struct: &CallbackStruct, closure: impl Fn(JNIEnv, &GlobalRef, Rust), rust_object: Rust) {
    if let Some(jvm) = &cb_struct.jvm {
        if let Some(callback) = &cb_struct.callback {
            let env = jvm
                .attach_current_thread_permanently()
                .unwrap_or_else(|err| {
                    error!(
                            "Error getting jvm in spawned thread for pairing state callback: {:?}",
                            err
                        );
                    panic!();
                });
            closure(env, callback, rust_object)
        }
    }
}

fn handle_state(state: WifiPairingState) {
    handle_callback(&STATE_CB_STRUCT.lock().unwrap(), move |env, callback, state| {
        let sig = [
            "(", WIFI_PAIRING_STATE_SIG, ")", VOID_SIG,
        ].concat();
        let java_state = JavaWifiPairingState(state);
        let state_class = CLASSREFSMAP::get_class_from_name(JavaWifiPairingState::full_name(Some(&java_state)));
        let state_value = java_state.j_object(env, state_class);
        invoke_callback(env, callback, sig, &[
            JValue::from(state_value)
        ]);
    }, state);
}

fn handle_networks(networks: Vec<WifiNetwork>) {
    handle_callback(&NETWORKS_CB_STRUCT.lock().unwrap(), move |env, callback, networks| {
        let sig = [
            "([", WIFI_NETWORK_SIG, ")", VOID_SIG,
        ].concat();
        let java_networks: Vec<JavaWifiNetwork> = networks.into_iter().map(|n| { JavaWifiNetwork(n) }).collect();
        let wifi_networks = AndroidList(java_networks).to_jobject(env);
        let value = JValue::from(JObject::from(wifi_networks));
        invoke_callback(env, callback, sig, &[
            value,
        ]);
    }, networks);
}

fn handle_result(result: Result<String, Box<MantleError>>) {
    handle_callback(&RESULT_CB_STRUCT.lock().unwrap(), move |env, callback, result| {
        let result_object = AndroidStringResult(result).to_jobject_result(env);
        invoke_result_callback(env, callback, result_object);
    }, result);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_startPairing(
    env: JNIEnv,
    _class: JClass,
    ptr_wifi_manager: jlong,
    j_ip_address: JString,
) {
    let wifi_manager = MantleJlong(ptr_wifi_manager).to_pointer::<WifiPairing>();
    let ip_address = MantleJString(j_ip_address).to_char_ptr(env);
    CLOUDCORE_API.cloudcore_start_pairing(wifi_manager, ip_address);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_continuePairing(
    _env: JNIEnv,
    _class: JClass,
    ptr_wifi_manager: jlong
) {
    let wifi_manager = MantleJlong(ptr_wifi_manager).to_pointer::<WifiPairing>();
    CLOUDCORE_API.cloudcore_continue_pairing(wifi_manager)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_donePairing(
    _env: JNIEnv,
    _class: JClass,
    ptr_wifi_manager: jlong
) {
    let wifi_manager = MantleJlong(ptr_wifi_manager).to_pointer::<WifiPairing>();
    CLOUDCORE_API.cloudcore_done_pairing(wifi_manager)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_handleSelectedNetwork(
    env: JNIEnv,
    _class: JClass,
    ptr_wifi_manager: jlong,
    j_wifi_network: JObject
) {
    if let Some(network) = JavaWifiNetwork::rust_object(MantleJObject(j_wifi_network), env) {
        let wifi_manager = MantleJlong(ptr_wifi_manager).to_pointer::<WifiPairing>();
        let boxed_network = Box::into_raw(Box::new(network));
        CLOUDCORE_API.cloudcore_handle_selected_network(wifi_manager, boxed_network);
    } else {
        debug!("selected network pointer was null");
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_writeToPairingLog(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_content: JString,
) -> jobject {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let content = MantleJString(j_content).to_char_ptr(env);
    let result = CLOUDCORE_API.cloudcore_write_to_pairing_log(cloudcore, content);
    *AndroidResult(result).to_jobject_result(env)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PairingKt_readPairingLog(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
) -> jobject {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let result: Result<String, Box<MantleError>> = CLOUDCORE_API.cloudcore_get_pairing_log(cloudcore);
    *AndroidStringResult(result).to_jobject_result(env)
}