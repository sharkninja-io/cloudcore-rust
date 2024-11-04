use cloudcore::{CloudCore, ErrorUtil};
use std::os::raw::c_char;
use ffi_utilities::{MantleStringPointer, RuntimeFFI};
use log::error;
use mantle_utilities::MantleError;
use cloudcore::schedules::Schedule;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_create_device_schedule(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    start_date: *const c_char,
    start_time_each_day: *const c_char,
    action_name: *const c_char,
    action_base_type: *const c_char,
    callback: fn(result: Result<Schedule, Box<MantleError>>),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let name = MantleStringPointer(name).to_string();
    let start_date = MantleStringPointer(start_date).to_string();
    let start_time_each_day = MantleStringPointer(start_time_each_day).to_string();
    let action_name = MantleStringPointer(action_name).to_string();
    let action_base_type = MantleStringPointer(action_base_type).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.create_device_schedule(dsn, name, start_date, start_time_each_day, action_name, action_base_type).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_fetch_schedules(
    ptr_cloudcore: *mut CloudCore,
    device_id: *const u32,
    callback: fn(result: Result<Vec<Schedule>, Box<MantleError>>),
) {
    let dev_id = *Box::from_raw(device_id as *mut u32);
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.fetch_schedules(Some(dev_id)).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_update_device_schedule(
    ptr_cloudcore: *mut CloudCore,
    schedule: *const Schedule,
    callback: fn(result: Result<Schedule, Box<MantleError>>),
) {
    if schedule.is_null() {
        error!("Invalid schedule object");
        callback(
            Err(Box::new(ErrorUtil::generic_error()))
        )
    } else {
        let schedule = *Box::from_raw(schedule as *mut Schedule);
        let cloudcore = &mut *ptr_cloudcore;
        let closure = async move {
            cloudcore.update_schedule(schedule).await
        };
        RuntimeFFI::exec(closure, callback);
    }
}