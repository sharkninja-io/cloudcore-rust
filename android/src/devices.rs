mod device;

use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jstring::MantleJString;
use android_utilities::{CallbackStruct, RuntimeAndroid, to_java_result, to_java_result_list};
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use jni::sys::{jboolean, jint, jlong};
use crate::devices::device::JavaIoTDevice;
use crate::cloudcore_ffi_api::CLOUDCORE_API;
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;
use cloudcore::devices::IoTDevice;
use cloudcore::CloudCore;

lazy_static! {
    static ref DEVICES_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DEVICE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref RENAME_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref RESET_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DEL_DEV_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DEL_MAP_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_fetchDevices(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEVICES_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_devices(cloudcore, handle_devices)
}

fn handle_devices(result: Result<Vec<IoTDevice>, Box<MantleError>>) {
    let cb_struct = DEVICES_CB_STRUCT.lock().unwrap();
    let result = to_java_result_list::<_, JavaIoTDevice>(result);
    RuntimeAndroid::exec_list(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_fetchDevice(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEVICE_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_device(cloudcore, dsn, handle_device)
}

fn handle_device(result: Result<IoTDevice, Box<MantleError>>) {
    let cb_struct = DEVICE_CB_STRUCT.lock().unwrap();
    let result = to_java_result::<_, JavaIoTDevice>(result);
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_renameDevice(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_new_name: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let new_name = MantleJString(j_new_name).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    RENAME_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_rename_device(cloudcore, dsn, new_name, handle_rename)
}

fn handle_rename(result: Result<(), Box<MantleError>>) {
    let cb_struct = RENAME_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_factoryResetDevice(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dev_id: jint,
    j_dsn: JString,
    j_callback: JObject,
) {
    let boxed_id = Box::into_raw(Box::new(j_dev_id as u32));
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    RESET_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_factory_reset_device(cloudcore, boxed_id, dsn, handle_reset)
}

fn handle_reset(result: Result<(), Box<MantleError>>) {
    let cb_struct = RESET_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_deleteDevice(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dev_id: jint,
    j_dsn: JString,
    j_callback: JObject,
) {
    let boxed_id = Box::into_raw(Box::new(j_dev_id as u32));
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEL_DEV_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_delete_device(cloudcore, boxed_id, dsn, handle_delete_device)
}

fn handle_delete_device(result: Result<(), Box<MantleError>>) {
    let cb_struct = DEL_DEV_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_DevicesKt_deleteMap(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_re_explore: jboolean,
    j_partial_delete: jboolean,
    j_callback: JObject,
) {
    let re_explore = Box::into_raw(Box::new(j_re_explore > 0));
    let partial_delete = Box::into_raw(Box::new(j_partial_delete > 0));
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEL_MAP_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_delete_device_map(cloudcore, dsn, re_explore, partial_delete, handle_delete_map)
}

fn handle_delete_map(result: Result<(), Box<MantleError>>) {
    let cb_struct = DEL_MAP_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}
