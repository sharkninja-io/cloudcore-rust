use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

use ffi_utilities::{convert_list_to_using_mantle_error, MantleStringPointer, RuntimeFFI};
use lazy_static::lazy_static;
use log::debug;

use mantle_utilities::{MantleError, RUNTIME};
use cloudcore::CloudCore;
use cloudcore::properties::trigger::IoTTrigger;

lazy_static! {
    static ref CREATE_TRIGGS_CB_STRUCT: Mutex<fn(result: Result<Vec<IoTTrigger>, Box<MantleError>>)> = Mutex::new(|_result|{});
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_fetch_triggers(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: Result<Vec<IoTTrigger>, Box<MantleError>>),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.fetch_triggers(dsn, prop_name).await
    };
    RuntimeFFI::exec_list(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_create_error_push_triggers(
    ptr_cloudcore: *mut CloudCore,
    device_nicknames: *mut HashMap<String, String>,
    device_id: *const c_char,
    application_id: *const c_char,
    channel_id: *const c_char,
    registration_id: *const c_char,
    service: *const c_char,
    errors: *mut HashMap<String, HashMap<u32, String>>,
    callback: fn(result: Result<Vec<IoTTrigger>, Box<MantleError>>),
) {
    debug!("cloudcore_create_error_push_triggers");
    let cloudcore = &mut *ptr_cloudcore;
    let device_nicknames = *Box::from_raw(device_nicknames);
    let device_id = MantleStringPointer(device_id).to_string();
    let application_id = MantleStringPointer(application_id).to_string();
    let service = MantleStringPointer(service).to_string();
    let errors = *Box::from_raw(errors);
    let channel_id = if channel_id.is_null() { None } else { Some(MantleStringPointer(channel_id).to_string()) };
    let registration_id = if registration_id.is_null() { None } else { Some(MantleStringPointer(registration_id).to_string()) };
    if let Some(mut cb) = CREATE_TRIGGS_CB_STRUCT.lock().ok() {
        *cb = callback;
    }
    debug!("cloudcore_create_error_push_triggers calling create_error_push_triggers_callback");
    let closure = async move {
        cloudcore.create_error_push_triggers_callback(
            device_nicknames,
            device_id,
            application_id,
            channel_id,
            registration_id,
            service,
            errors,
            |result| {
                if let Some(cb) = CREATE_TRIGGS_CB_STRUCT.lock().ok() {
                    cb(convert_list_to_using_mantle_error(result))
                }
            }
        ).await
    };
    RUNTIME.spawn(closure);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_all_triggers(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let closure = async move {
        cloudcore.delete_all_triggers(dsn, prop_name).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_update_all_trigger_apps_registration_id(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    device_id: *const c_char,
    registration_id: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let device_id = MantleStringPointer(device_id).to_string();
    let registration_id = MantleStringPointer(registration_id).to_string();
    let closure = async move {
        cloudcore.update_all_trigger_apps_registration_id(
            dsn,
            prop_name,
            device_id,
            registration_id,
        ).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_all_trigger_apps_by_device_id(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    device_id: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let device_id = MantleStringPointer(device_id).to_string();
    let closure = async move {
        cloudcore.delete_all_trigger_apps_by_device_id(
            dsn,
            prop_name,
            device_id
        ).await
    };
    RuntimeFFI::exec(closure, callback);
}