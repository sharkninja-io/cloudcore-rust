mod property;
mod value;
mod datapoint;

use std::os::raw::{c_char, c_int};
use cloudcore::CloudCore;
use ffi_utilities::{CRustBridge, MantleList, MantleResult, MantleString, MantleStringPointer};
use ios_utilities::{CallbackStruct, CallbackStructMap, ListCallbackStruct, ListCallbackStructMap};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;
use cloudcore::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMessage};
use cloudcore::properties::property::IoTProperty;
use cloudcore::properties::value::IoTPropertyValue;
use crate::properties::datapoint::{IoTDatapoint as iOSIoTDatapoint, IoTDatapointFile as iOSIoTDatapointFile, IoTDatapointMessage as iOSIoTDatapointMessage};
use crate::properties::property::IoTProperty as iOSIoTProperty;
use crate::properties::value::IoTPropertyValue as iOSIoTPropertyValue;

lazy_static! {
    // TODO: statics with the same signature can be consolidated
    static ref PROP_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTProperty>> = Mutex::new(ListCallbackStructMap::new());
    static ref PROPS_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTProperty>> = Mutex::new(ListCallbackStructMap::new());
    static ref DATA_POINTS_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTDatapoint>> = Mutex::new(ListCallbackStructMap::new());
    static ref FILE_CB_STRUCT: Mutex<CallbackStructMap<iOSIoTDatapointFile>> = Mutex::new(CallbackStructMap::new());
    static ref MULTI_FILE_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTDatapointFile>> = Mutex::new(ListCallbackStructMap::new());
    static ref FILE_DATAPOINT_CB_STRUCT: Mutex<CallbackStructMap<iOSIoTDatapointFile>> = Mutex::new(CallbackStructMap::new());
    static ref MSG_CB_STRUCT: Mutex<CallbackStructMap<iOSIoTDatapointMessage>> = Mutex::new(CallbackStructMap::new());
    static ref MULTI_MSG_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTDatapointMessage>> = Mutex::new(ListCallbackStructMap::new());
    static ref MSG_DATAPOINT_CB_STRUCT: Mutex<CallbackStructMap<iOSIoTDatapointMessage>> = Mutex::new(CallbackStructMap::new());
    static ref SET_PROP_CB_STRUCT: Mutex<CallbackStructMap<()>> = Mutex::new(CallbackStructMap::new());
    static ref FILES_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTDatapointFile>> = Mutex::new(ListCallbackStructMap::new());
    static ref MSGS_CB_STRUCT: Mutex<ListCallbackStructMap<iOSIoTDatapointMessage>> = Mutex::new(ListCallbackStructMap::new());
    static ref SAVE_FILE_CB_STRUCT: Mutex<CallbackStructMap<()>> = Mutex::new(CallbackStructMap::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_get_property(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_properties(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_data_points(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapoint>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_file_property(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
    );
    fn cloudcore_get_file_properties(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_datapoint_with_file_url(
        ptr_cloudcore: *mut CloudCore,
        url: *const c_char,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
    );
    fn cloudcore_get_message_property(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
    );
    fn cloudcore_get_message_properties(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_datapoint_with_id(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        datapoint_id: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
    );
    fn cloudcore_set_property_value(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        value: *mut IoTPropertyValue,
        callback_id: *const c_char,
        callback: fn(result: (Result<(), Box<MantleError>>, String)),
    );
    fn cloudcore_get_file_property_as_files(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
    );
    fn cloudcore_get_message_property_as_files(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
    );
    fn cloudcore_save_file(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        path: *const c_char,
        is_message: *const bool,
        callback_id: *const c_char,
        callback: fn(result: (Result<(), Box<MantleError>>, String)),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    callback: fn(result: MantleResult<MantleList<iOSIoTProperty>>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = PROP_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTProperty> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_property(ptr_cloudcore, dsn, name, struct_callback_id, handle_prop);
}

fn handle_prop(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)) {
    PROP_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    ptr_names: *const MantleList<*const c_char>,
    callback: fn(result: MantleResult<MantleList<iOSIoTProperty>>, callback_id: u64),
    callback_id: u64,
) {
    let c_list = &*ptr_names;
    let names = c_list.map_list(|c_name|{ MantleStringPointer(c_name).to_string() });
    let boxed_names = Box::into_raw(Box::new(names));
    let mut cb_struct = PROPS_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTProperty> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_properties(ptr_cloudcore, dsn, boxed_names, struct_callback_id, handle_props);
}

fn handle_props(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)) {
    PROPS_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_data_points(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback: fn(result: MantleResult<MantleList<iOSIoTDatapoint>>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = DATA_POINTS_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTDatapoint> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_data_points(ptr_cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_data_points);
}

fn handle_data_points(result: (Result<Vec<IoTDatapoint>, Box<MantleError>>, String)) {
    DATA_POINTS_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_file_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: MantleResult<iOSIoTDatapointFile>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<iOSIoTDatapointFile> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_file_property(ptr_cloudcore, dsn, prop_name, struct_callback_id, handle_file);
}

fn handle_file(result: (Result<IoTDatapointFile, Box<MantleError>>, String)) {
    FILE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_file_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    ptr_names: *const MantleList<*const c_char>,
    callback: fn(result: MantleResult<MantleList<iOSIoTDatapointFile>>, callback_id: u64),
    callback_id: u64,
) {
    let c_list = &*ptr_names;
    let names = c_list.map_list(|c_name|{ MantleStringPointer(c_name).to_string() });
    let boxed_names = Box::into_raw(Box::new(names));
    let mut cb_struct = MULTI_FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTDatapointFile> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_file_properties(ptr_cloudcore, dsn, boxed_names, struct_callback_id, handle_mutli_files);
}

fn handle_mutli_files(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)) {
    MULTI_FILE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_file_property_with_url(
    ptr_cloudcore: *mut CloudCore,
    url: *const c_char,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: MantleResult<iOSIoTDatapointFile>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = FILE_DATAPOINT_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<iOSIoTDatapointFile> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_datapoint_with_file_url(ptr_cloudcore, url, dsn, prop_name, struct_callback_id, handle_file_datapoint);
}

fn handle_file_datapoint(result: (Result<IoTDatapointFile, Box<MantleError>>, String)) {
    FILE_DATAPOINT_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_message_property(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: MantleResult<iOSIoTDatapointMessage>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = MSG_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<iOSIoTDatapointMessage> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_message_property(ptr_cloudcore, dsn, prop_name, struct_callback_id, handle_msg);
}

fn handle_msg(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)) {
    MSG_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_message_properties(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    ptr_names: *const MantleList<*const c_char>,
    callback: fn(result: MantleResult<MantleList<iOSIoTDatapointMessage>>, callback_id: u64),
    callback_id: u64,
) {
    let c_list = &*ptr_names;
    let names = c_list.map_list(|c_name|{ MantleStringPointer(c_name).to_string() });
    let boxed_names = Box::into_raw(Box::new(names));
    let mut cb_struct = MULTI_MSG_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTDatapointMessage> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_message_properties(ptr_cloudcore, dsn, boxed_names, struct_callback_id, handle_mutli_msgs);
}

fn handle_mutli_msgs(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)) {
    MULTI_MSG_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_datapoint_with_id(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    datapoint_id: *const c_char,
    prop_name: *const c_char,
    callback: fn(result: MantleResult<iOSIoTDatapointMessage>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = MSG_DATAPOINT_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<iOSIoTDatapointMessage> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_datapoint_with_id(ptr_cloudcore, dsn, datapoint_id, prop_name, struct_callback_id, handle_msg_datapoint);
}

fn handle_msg_datapoint(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)) {
    MSG_DATAPOINT_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_set_property_value(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    ptr_value: *const iOSIoTPropertyValue,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    let value = iOSIoTPropertyValue::new_rust_object(ptr_value).unwrap();
    let boxed_value = Box::into_raw(Box::new(value));
    let mut cb_struct = SET_PROP_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<()> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_set_property_value(ptr_cloudcore, dsn, name, boxed_value, struct_callback_id, handle_set_prop);
}

fn handle_set_prop(result: (Result<(), Box<MantleError>>, String)) {
    SET_PROP_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_file_property_as_files(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback: fn(result: MantleResult<MantleList<iOSIoTDatapointFile>>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = FILES_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTDatapointFile> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_file_property_as_files(ptr_cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_files);
}

fn handle_files(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)) {
    FILES_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_get_message_property_as_files(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    prop_name: *const c_char,
    count: *const c_int,
    from: *const c_char,
    to: *const c_char,
    callback: fn(result: MantleResult<MantleList<iOSIoTDatapointMessage>>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = MSGS_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: ListCallbackStruct<iOSIoTDatapointMessage> = ListCallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    cloudcore_get_message_property_as_files(ptr_cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_messages);
}

fn handle_messages(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)) {
    MSGS_CB_STRUCT.lock().unwrap().run(result);
}


#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_save_file(
    ptr_cloudcore: *mut CloudCore,
    dsn: *const c_char,
    name: *const c_char,
    path: *const c_char,
    is_message: bool,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    let mut cb_struct = SAVE_FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback: CallbackStruct<()> = CallbackStruct::new();
    struct_callback.update(callback, callback_id);
    cb_struct.update(struct_callback, next_id.clone());
    let is_message = Box::into_raw(Box::new(is_message));
    cloudcore_save_file(ptr_cloudcore, dsn, name, path, is_message, struct_callback_id, handle_save_file);
}

fn handle_save_file(result: (Result<(), Box<MantleError>>, String)) {
    SAVE_FILE_CB_STRUCT.lock().unwrap().run(result);
}
