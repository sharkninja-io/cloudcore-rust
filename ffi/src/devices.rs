use cloudcore::{CloudCore};
use std::os::raw::c_char;
use ffi_utilities::{MantleStringPointer, RuntimeFFI};
use mantle_utilities::MantleError;
use cloudcore::devices::IoTDevice;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_devices(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<Vec<IoTDevice>, Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.fetch_all_devices().await
    };
    RuntimeFFI::exec_list(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_device(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    callback: fn(result: Result<IoTDevice, Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let closure = async move {
        cloudcore.fetch_device_with_dsn(dsn).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_rename_device(
    ptr_cloudcore: *const CloudCore,
    dsn: *const c_char,
    new_name: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = & *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let new_name = MantleStringPointer(new_name).to_string();
    let closure = async move {
        cloudcore.rename_device_with_dsn(dsn, new_name).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_factory_reset_device(
    ptr_cloudcore: *mut CloudCore,
    device_id: *const u32,
    dsn: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let dev_id = *Box::from_raw(device_id as *mut u32);
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.factory_reset_device(dev_id, dsn).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_device(
    ptr_cloudcore: *mut CloudCore,
    device_id: *const u32,
    dsn: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let dev_id = *Box::from_raw(device_id as *mut u32);
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.delete_device(dev_id, dsn).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_device_map(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    re_explore: *const bool,
    partial_delete: *const bool,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let re_explore = *Box::from_raw(re_explore as *mut bool);
    let partial_delete = *Box::from_raw(partial_delete as *mut bool);
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.delete_device_map(dsn, re_explore, partial_delete).await
    };
    RuntimeFFI::exec(closure, callback);
}