use cloudcore::CloudCore;
use std::os::raw::{c_char, c_int};
use ffi_utilities::{convert_list_to_using_mantle_error_with_id, MantleStringPointer, RuntimeFFI};
use mantle_utilities::{MantleError, RUNTIME};
use cloudcore::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMessage};
use cloudcore::properties::property::IoTProperty;
use cloudcore::properties::value::IoTPropertyValue;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PROP_MSGS_CB_STRUCT: Mutex<fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String))> = Mutex::new(|_result|{});
    static ref PROP_FILES_CB_STRUCT: Mutex<fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String))> = Mutex::new(|_result|{});
    static ref MSGS_CB_STRUCT: Mutex<fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String))> = Mutex::new(|_result|{});
    static ref FILES_CB_STRUCT: Mutex<fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String))> = Mutex::new(|_result|{});
}
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let prop = MantleStringPointer(name).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_property(dsn, prop, callback_id).await
    };
    RuntimeFFI::exec_list_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    names: *mut Vec<String>,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let props = *Box::from_raw(names);
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_properties(dsn, props, callback_id).await
    };
    RuntimeFFI::exec_list_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_data_points(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTDatapoint>, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let count = if count.is_null() { None } else { Some(count as u32) };
    let from = MantleStringPointer(from).to_option_string();
    let to = MantleStringPointer(to).to_option_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_datapoints(dsn, prop_name, count, from, to, callback_id).await
    };
    RuntimeFFI::exec_list_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_file_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_file_property(dsn, prop_name, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_file_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    names: *mut Vec<String>,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let props = *Box::from_raw(names);
    let callback_id = MantleStringPointer(callback_id).to_string();
    if let Some(mut cb) = FILES_CB_STRUCT.lock().ok() {
        *cb = callback;
    }
    let closure = async move {
        cloudcore.get_file_properties_callback(dsn, props, callback_id, |result| {
            if let Some(cb) = FILES_CB_STRUCT.lock().ok() {
                cb(convert_list_to_using_mantle_error_with_id(result))
            }
        }).await
    };
    RUNTIME.spawn(closure);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_datapoint_with_file_url(
    ptr_cloudcore: *mut CloudCore,
    url: *const c_char,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let url = MantleStringPointer(url).to_string();
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_datapoint_with_file_url(url, dsn, prop_name, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_message_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_message_property(dsn, prop_name, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_message_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    names: *mut Vec<String>,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
) {
    let dsn = MantleStringPointer(dsn).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let props = *Box::from_raw(names);
    let callback_id = MantleStringPointer(callback_id).to_string();
    if let Some(mut cb) = MSGS_CB_STRUCT.lock().ok() {
        *cb = callback;
    }
    let closure = async move {
        cloudcore.get_message_properties_callback(dsn, props, callback_id, |result| {
            if let Some(cb) = MSGS_CB_STRUCT.lock().ok() {
                cb(convert_list_to_using_mantle_error_with_id(result))
            }
        }).await
    };
    RUNTIME.spawn(closure);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_datapoint_with_id(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    datapoint_id: *const c_char,
    prop_name: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let datapoint_id = MantleStringPointer(datapoint_id).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.get_datapoint_with_id(dsn, datapoint_id, prop_name, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_set_property_value(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    value: *mut IoTPropertyValue,
    callback_id: *const c_char,
    callback: fn(result: (Result<(), Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let name = MantleStringPointer(name).to_string();
    let value = *Box::from_raw(value);
    let callback_id = MantleStringPointer(callback_id).to_string();

    let closure = async move {
        cloudcore.set_property_value(dsn, name, value, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}


#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_file_property_as_files(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let count = if count.is_null() { None } else { Some(count as u32) };
    let from = MantleStringPointer(from).to_option_string();
    let to = MantleStringPointer(to).to_option_string();
    let callback_id = MantleStringPointer(callback_id).to_string();
    if let Some(mut cb) = PROP_FILES_CB_STRUCT.lock().ok() {
        *cb = callback;
    }
    let closure = async move {
        cloudcore.get_file_property_as_files_callback(dsn, prop_name, count, from, to, callback_id, |result| {
            if let Some(cb) = PROP_FILES_CB_STRUCT.lock().ok() {
                cb(convert_list_to_using_mantle_error_with_id(result))
            }
        }).await
    };
    RUNTIME.spawn(closure);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_message_property_as_files(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback_id: *const c_char,
    callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let prop_name = MantleStringPointer(prop_name).to_string();
    let count = if count.is_null() { None } else { Some(count as u32) };
    let from = MantleStringPointer(from).to_option_string();
    let to = MantleStringPointer(to).to_option_string();
    let callback_id = MantleStringPointer(callback_id).to_string();
    if let Some(mut cb) = PROP_MSGS_CB_STRUCT.lock().ok() {
        *cb = callback;
    }
    let closure = async move {
        cloudcore.get_message_property_as_files_callback(dsn, prop_name, count, from, to, callback_id, |result| {
            if let Some(cb) = PROP_MSGS_CB_STRUCT.lock().ok() {
                cb(convert_list_to_using_mantle_error_with_id(result))
            }
        }).await
    };
    RUNTIME.spawn(closure);
}
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_save_file(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    path: *const c_char,
    is_message: *const bool,
    callback_id: *const c_char,
    callback: fn(result: (Result<(), Box<MantleError>>, String)),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let dsn = MantleStringPointer(dsn).to_string();
    let name = MantleStringPointer(name).to_string();
    let path = MantleStringPointer(path).to_string();
    let callback_id = MantleStringPointer(callback_id).to_string();
    let is_message = *Box::from_raw(is_message as *mut bool);

    let closure = async move {
        cloudcore.save_file(dsn, name, path, is_message, callback_id).await
    };
    RuntimeFFI::exec_id(closure, callback);
}

