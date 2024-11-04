use std::os::raw::c_char;

use ffi_utilities::{MantleStringPointer, RuntimeFFI};
use mantle_utilities::MantleError;

use cloudcore::CloudCore;
use cloudcore::notifications::notifications::Notification;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_fetch_all_notifications(
    ptr_cloudcore: *mut CloudCore,
    from: *const c_char,
    callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let from = MantleStringPointer(from).to_string();

    let closure = async move {
        cloudcore.fetch_all_notifications(from).await
    };
    RuntimeFFI::exec_list(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_cached_notifications(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.get_all_cached_notifications().await
    };
    RuntimeFFI::exec_list(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_all_notifications(
    ptr_cloudcore: *mut CloudCore,
    to: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let to = MantleStringPointer(to).to_string();
    let closure = async move {
        cloudcore.delete_all_notifications(to).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_notification(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    id: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let id = MantleStringPointer(id).to_string();
    let closure = async move {
        cloudcore.delete_notification(dsn, id).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_mark_all_notifications_as_read(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.mark_all_notifications_as_read().await
    };
    RuntimeFFI::exec(closure, callback);
}