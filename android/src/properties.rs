mod property;
pub mod value;
mod datapoint;

use std::os::raw::c_int;
use std::ptr;
use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jobject::{get_jint, MantleJObject};
use android_utilities::jni_exts::jobject_array::MantleJObjectArray;
use android_utilities::jni_exts::jstring::MantleJString;
use android_utilities::{CallbackStruct, CallbackStructMap, JObjectRustBridge, RuntimeAndroid, to_java_result, to_java_result_list};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, jobjectArray};
use crate::cloudcore_ffi_api::CLOUDCORE_API;
use jni::JNIEnv;
use cloudcore::CloudCore;
use lazy_static::lazy_static;
use std::sync::Mutex;
use ffi_utilities::MantleString;
use mantle_utilities::MantleError;
use cloudcore::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMessage};
use cloudcore::properties::property::IoTProperty;
use crate::properties::datapoint::{JavaIoTDatapoint, JavaIoTDatapointFile, JavaIoTDatapointMessage};
use crate::properties::property::JavaIoTProperty;
use crate::properties::value::JavaIoTPropertyValue;

lazy_static! {
    static ref GET_PROP_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref GET_PROPS_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref DATA_POINTS_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref FILE_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref MULTI_FILE_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref FILE_DATAPOINT_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref MSG_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref MULTI_MSG_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref MSG_DATAPOINT_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref SET_PROP_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref SAVE_FILE_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref FILES_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
    static ref MSG_PROPERTY_AS_FILES_CB_STRUCT: Mutex<CallbackStructMap> = Mutex::new(CallbackStructMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getProperty(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_name: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop = MantleJString(j_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = GET_PROP_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_property(cloudcore, dsn, prop, struct_callback_id, handle_prop);
}

fn handle_prop(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTProperty>(result.0);
    if let Some(cb_struct)  = GET_PROP_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getProperties(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_names: jobjectArray,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let props = MantleJObjectArray(j_names).to_list::<String,String>(env);
    let boxed_props = Box::into_raw(Box::new(props));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = GET_PROPS_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_properties(cloudcore, dsn, boxed_props, struct_callback_id, handle_props);
}

fn handle_props(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTProperty>(result.0);
    if let Some(cb_struct)  = GET_PROPS_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getDataPoints(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_prop_name: JString,
    j_count: JObject,
    j_from: JString,
    j_to: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let count: *const c_int = if j_count.is_null() { ptr::null() } else { &(get_jint(j_count, env) as c_int) };
    let from = MantleJString(j_from).to_char_ptr(env);
    let to = MantleJString(j_to).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = DATA_POINTS_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_data_points(cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_data_points);
}

fn handle_data_points(result: (Result<Vec<IoTDatapoint>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTDatapoint>(result.0);
    if let Some(cb_struct) = DATA_POINTS_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getFileProperty(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_prop_name: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_file_property(cloudcore, dsn, prop_name, struct_callback_id, handle_file);
}

fn handle_file(result: (Result<IoTDatapointFile, Box<MantleError>>, String)) {
    let res = to_java_result::<_, JavaIoTDatapointFile>(result.0);
    if let Some(cb_struct) = FILE_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getMultipleFileProperties(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_names: jobjectArray,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let props = MantleJObjectArray(j_names).to_list::<String,String>(env);
    let boxed_props = Box::into_raw(Box::new(props));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = MULTI_FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_file_properties(cloudcore, dsn, boxed_props, struct_callback_id, handle_multi_files);
}

fn handle_multi_files(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTDatapointFile>(result.0);
    if let Some(cb_struct) = MULTI_FILE_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getFilePropertyDatapoint(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_url: JString,
    j_dsn: JString,
    j_prop_name: JString,
    j_callback: JObject,
) {
    let url = MantleJString(j_url).to_char_ptr(env);
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = FILE_DATAPOINT_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_datapoint_with_file_url(cloudcore, url, dsn, prop_name, struct_callback_id, handle_file_datapoint);
}

fn handle_file_datapoint(result: (Result<IoTDatapointFile, Box<MantleError>>, String)) {
    let res = to_java_result::<_,JavaIoTDatapointFile>(result.0);
    if let Some(cb_struct) = FILE_DATAPOINT_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getMessageProperty(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_prop_name: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = MSG_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_message_property(cloudcore, dsn, prop_name, struct_callback_id, handle_message);
}

fn handle_message(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)) {
    let res = to_java_result::<_,JavaIoTDatapointMessage>(result.0);
    if let Some(cb_struct) = MSG_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getMultipleMessageProperties(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_names: jobjectArray,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let props = MantleJObjectArray(j_names).to_list::<String, String>(env);
    let boxed_props = Box::into_raw(Box::new(props));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = MULTI_MSG_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_message_properties(cloudcore, dsn, boxed_props, struct_callback_id, handle_multi_msg);
}

fn handle_multi_msg(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTDatapointMessage>(result.0);
    if let Some(cb_struct) = MULTI_MSG_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getMessagePropertyDatapoint(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_datapoint_id: JString,
    j_prop_name: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let datapoint_id = MantleJString(j_datapoint_id).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = MSG_DATAPOINT_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_datapoint_with_id(cloudcore, dsn, datapoint_id, prop_name, struct_callback_id, handle_msg_datapoint);
}

fn handle_msg_datapoint(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)) {
    let res = to_java_result::<_,JavaIoTDatapointMessage>(result.0);
    if let Some(cb_struct) = MSG_DATAPOINT_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_setPropertyValue(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_name: JString,
    j_value: JObject,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let name = MantleJString(j_name).to_char_ptr(env);
    let value = JavaIoTPropertyValue::rust_object(MantleJObject(j_value), env).unwrap();
    let boxed_value = Box::into_raw(Box::new(value));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = SET_PROP_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_set_property_value(cloudcore, dsn, name, boxed_value, struct_callback_id, handle_set_prop);
}

fn handle_set_prop(result: (Result<(), Box<MantleError>>, String)) {
    if let Some(cb_struct) = SET_PROP_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, result.0, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getFilePropertyAsFiles(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_prop_name: JString,
    j_count: JObject,
    j_from: JString,
    j_to: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let count: *const c_int = if j_count.is_null() { ptr::null() } else { &(get_jint(j_count, env) as c_int) };
    let from = MantleJString(j_from).to_char_ptr(env);
    let to = MantleJString(j_to).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = FILES_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_file_property_as_files(cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_files);
}

fn handle_files(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_, JavaIoTDatapointFile>(result.0);
    if let Some(cb_struct) = FILES_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_getMessagePropertyAsFiles(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_prop_name: JString,
    j_count: JObject,
    j_from: JString,
    j_to: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let prop_name = MantleJString(j_prop_name).to_char_ptr(env);
    let count: *const c_int = if j_count.is_null() { ptr::null() } else { &(get_jint(j_count, env) as c_int) };
    let from = MantleJString(j_from).to_char_ptr(env);
    let to = MantleJString(j_to).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = MSG_PROPERTY_AS_FILES_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_get_message_property_as_files(cloudcore, dsn, prop_name, count, from, to, struct_callback_id, handle_message_property_as_files);
}

fn handle_message_property_as_files(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)) {
    let res = to_java_result_list::<_,JavaIoTDatapointMessage>(result.0);
    if let Some(cb_struct) = MSG_PROPERTY_AS_FILES_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec_list(&cb_struct.jvm, res, &cb_struct.callback);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_PropertiesKt_saveFile(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_name: JString,
    j_path: JString,
    j_is_message: jboolean,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let name = MantleJString(j_name).to_char_ptr(env);
    let path = MantleJString(j_path).to_char_ptr(env);
    let is_msg: bool = j_is_message > 0;
    let is_message = Box::into_raw(Box::new(is_msg));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let mut cb_struct = SAVE_FILE_CB_STRUCT.lock().unwrap();
    let next_id = cb_struct.get_next_id_as_string();
    let struct_callback_id = MantleString(next_id.clone()).to_ptr();
    let mut struct_callback = CallbackStruct::new();
    struct_callback.update(env, j_callback);
    cb_struct.update(struct_callback, next_id.clone());
    CLOUDCORE_API.cloudcore_save_file(cloudcore, dsn, name, path, is_message, struct_callback_id, handle_save_file);
}

fn handle_save_file(result: (Result<(), Box<MantleError>>, String)) {
    if let Some(cb_struct) = SAVE_FILE_CB_STRUCT.lock().unwrap().get(result.1) {
        RuntimeAndroid::exec(&cb_struct.jvm, result.0, &cb_struct.callback);
    }
}

