use std::os::raw::c_char;
use ffi_utilities::{MantleList, MantleResult};
use cloudcore::CloudCore;
use cloudcore::notifications::notifications::Notification;
use crate::notifications::notification::iOSNotification;
use ios_utilities::{ListCallbackStruct, CallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;

mod notification;

lazy_static! {
    static ref FETCH_CB_STRUCT: Mutex<ListCallbackStruct<iOSNotification>> = Mutex::new(ListCallbackStruct::new());
    static ref CACHED_CB_STRUCT: Mutex<ListCallbackStruct<iOSNotification>> = Mutex::new(ListCallbackStruct::new());
    static ref DELETE_ALL_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref DELETE_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref READ_ALL_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_fetch_all_notifications(
        ptr_cloudcore: *mut CloudCore,
        from: *const c_char,
        callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
    );
    fn cloudcore_get_cached_notifications(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
    );
    fn cloudcore_delete_all_notifications(
        ptr_cloudcore: *mut CloudCore,
        to: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_delete_notification(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        id: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_mark_all_notifications_as_read(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_fetch_all_notifications(
    ptr_cloudcore: *mut CloudCore,
    from: *const c_char,
    callback: fn(result: MantleResult<MantleList<iOSNotification>>, callback_id: u64),
    callback_id: u64,
) {
    FETCH_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_fetch_all_notifications(ptr_cloudcore, from, handle_fetch);
}

fn handle_fetch(result: Result<Vec<Notification>, Box<MantleError>>) {
    FETCH_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_cached_notifications(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<MantleList<iOSNotification>>, callback_id: u64),
    callback_id: u64,
) {
    CACHED_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_get_cached_notifications(ptr_cloudcore, handle_cached);
}

fn handle_cached(result: Result<Vec<Notification>, Box<MantleError>>) {
    CACHED_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_delete_all_notifications(
    ptr_cloudcore: *mut CloudCore,
    to: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    DELETE_ALL_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_delete_all_notifications(ptr_cloudcore, to, handle_delete_all);
}

fn handle_delete_all(result: Result<(), Box<MantleError>>) {
    DELETE_ALL_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_delete_notification(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    id: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    DELETE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_delete_notification(ptr_cloudcore, dsn, id, handle_delete);
}

fn handle_delete(result: Result<(), Box<MantleError>>) {
    DELETE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_mark_all_notifications_as_read(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    READ_ALL_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_mark_all_notifications_as_read(ptr_cloudcore, handle_read_all);
}

fn handle_read_all(result: Result<(), Box<MantleError>>) {
    READ_ALL_CB_STRUCT.lock().unwrap().run(result);
}