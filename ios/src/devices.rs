mod device;

use std::os::raw::c_char;
use ffi_utilities::{MantleList, MantleResult};
use ios_utilities::{CallbackStruct, ListCallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;
use cloudcore::devices::IoTDevice;
use crate::devices::device::IoTDevice as iOSIoTDevice;
use cloudcore::CloudCore;

lazy_static! {
    static ref DEVICES_CB_STRUCT: Mutex<ListCallbackStruct<iOSIoTDevice>> = Mutex::new(ListCallbackStruct::new());
    static ref DEVICE_CB_STRUCT: Mutex<CallbackStruct<iOSIoTDevice>> = Mutex::new(CallbackStruct::new());
    static ref RENAME_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref RESET_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref DEL_DEV_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref DEL_MAP_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_devices(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<Vec<IoTDevice>, Box<MantleError>>),
    );
    fn cloudcore_device(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        callback: fn(result: Result<IoTDevice, Box<MantleError>>),
    );
    fn cloudcore_rename_device(
        ptr_cloudcore: *const CloudCore,
        dsn: *const c_char,
        new_name: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_factory_reset_device(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        dsn: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_delete_device(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        dsn: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_delete_device_map(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        re_explore: *const bool,
        partial_delete: *const bool,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_devices(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<MantleList<iOSIoTDevice>>, callback_id: u64),
    callback_id: u64,
) {
    DEVICES_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_devices(ptr_cloudcore, handle_devices)
}

fn handle_devices(result: Result<Vec<IoTDevice>, Box<MantleError>>) {
    DEVICES_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_device(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    callback: fn(result: MantleResult<iOSIoTDevice>, callback_id: u64),
    callback_id: u64,
) {
    DEVICE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_device(ptr_cloudcore, dsn, handle_device)
}

fn handle_device(result: Result<IoTDevice, Box<MantleError>>) {
    DEVICE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_rename_device(
    ptr_cloudcore: *const CloudCore,
    dsn: *const c_char,
    new_name: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    RENAME_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_rename_device(ptr_cloudcore, dsn, new_name, handle_rename)
}

fn handle_rename(result: Result<(), Box<MantleError>>) {
    RENAME_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_factory_reset_device(
    ptr_cloudcore: *mut CloudCore,
    device_id: u32,
    dsn: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    RESET_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    let id = Box::into_raw(Box::new(device_id));
    cloudcore_factory_reset_device(ptr_cloudcore, id, dsn, handle_reset)
}

fn handle_reset(result: Result<(), Box<MantleError>>) {
    RESET_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_delete_device(
    ptr_cloudcore: *mut CloudCore,
    device_id: u32,
    dsn: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    DEL_DEV_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    let id = Box::into_raw(Box::new(device_id));
    cloudcore_delete_device(ptr_cloudcore, id, dsn, handle_delete_device)
}

fn handle_delete_device(result: Result<(), Box<MantleError>>) {
    DEL_DEV_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_delete_map(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    re_explore: bool,
    partial_delete: bool,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    DEL_MAP_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    let re_explore = Box::into_raw(Box::new(re_explore));
    let partial_delete = Box::into_raw(Box::new(partial_delete));
    cloudcore_delete_device_map(ptr_cloudcore, dsn, re_explore, partial_delete, handle_delete_map)
}

fn handle_delete_map(result: Result<(), Box<MantleError>>) {
    DEL_MAP_CB_STRUCT.lock().unwrap().run(result);
}