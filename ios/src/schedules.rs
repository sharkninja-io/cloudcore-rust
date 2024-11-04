mod schedule;

use cloudcore::CloudCore;
use std::os::raw::c_char;
use ffi_utilities::{MantleList, MantleResult, CRustBridge};
use ios_utilities::{CallbackStruct, ListCallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::schedules::schedule::Schedule as iOSSchedule;
use cloudcore::schedules::Schedule;
use log::error;
use mantle_utilities::MantleError;

lazy_static! {
    static ref CREATE_DEVICE_SCHEDULE_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref DEVICE_SCHEDULES_LIST_CB_STRUCT: Mutex<ListCallbackStruct<iOSSchedule>> = Mutex::new(ListCallbackStruct::new());
    static ref DEVICE_UPDATE_SCHEDULE_CB_STRUCT: Mutex<CallbackStruct<iOSSchedule>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_create_device_schedule(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        start_date: *const c_char,
        action_name: *const c_char,
        action_base_type: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_fetch_schedules(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        callback: fn(result: Result<Vec<Schedule>, Box<MantleError>>),
    );
    fn cloudcore_update_device_schedule(
        ptr_cloudcore: *mut CloudCore,
        schedule: *mut Schedule,
        callback: fn(result: Result<Schedule, Box<MantleError>>),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_create_device_schedule(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    start_date: *const c_char,
    action_name: *const c_char,
    action_base_type: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    CREATE_DEVICE_SCHEDULE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_create_device_schedule(ptr_cloudcore, dsn, name, start_date, action_name, action_base_type, handle_create);
}

fn handle_create(result: Result<(), Box<MantleError>>) {
    CREATE_DEVICE_SCHEDULE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_fetch_schedules(
    ptr_cloudcore: *mut CloudCore,
    device_id: u32,
    callback: fn(result: MantleResult<MantleList<iOSSchedule>>, callback_id: u64),
    callback_id: u64,
) {
    DEVICE_SCHEDULES_LIST_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    let dev_id = Box::into_raw(Box::new(device_id));
    cloudcore_fetch_schedules(ptr_cloudcore, dev_id, handle_schedules)
}

fn handle_schedules(result: Result<Vec<Schedule>, Box<MantleError>>) {
    DEVICE_SCHEDULES_LIST_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_update_device_schedule(
    ptr_cloudcore: *mut CloudCore,
    schedule: *const iOSSchedule,
    callback: fn(result: MantleResult<iOSSchedule>, callback_id: u64),
    callback_id: u64,
) {
    if let Some(raw_schedule) = iOSSchedule::new_rust_object(schedule) {
        let boxed_schedule = Box::into_raw(Box::new(raw_schedule));
        DEVICE_UPDATE_SCHEDULE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
        cloudcore_update_device_schedule(ptr_cloudcore, boxed_schedule, handle_update);
    } else {
        error!("passed in iOS network object could not be converted to Rust");
    }
}

fn handle_update(result: Result<Schedule, Box<MantleError>>) {
    DEVICE_UPDATE_SCHEDULE_CB_STRUCT.lock().unwrap().run(result);
}