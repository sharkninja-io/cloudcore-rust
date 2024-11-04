mod schedule;

use android_utilities::jni_exts::jlong::MantleJlong;
use jni::objects::{JClass, JObject};
use jni::JNIEnv;
use jni::sys::{jint, jlong};
use crate::cloudcore_ffi_api::CLOUDCORE_API;
use lazy_static::lazy_static;
use std::sync::Mutex;
use android_utilities::{CallbackStruct, JObjectRustBridge, RuntimeAndroid, to_java_result, to_java_result_list};
use android_utilities::jni_exts::jobject::MantleJObject;
use mantle_utilities::MantleError;
use cloudcore::CloudCore;
use cloudcore::schedules::Schedule;
use crate::schedules::schedule::JavaSchedule;

lazy_static! {
    static ref CREATE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref FETCH_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref UPDATE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_SchedulesKt_fetchDeviceSchedules(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_device_id: jint,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let device_id = Box::into_raw(Box::new(j_device_id as u32));
    FETCH_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_fetch_schedules(cloudcore, device_id, handle_fetch)
}

fn handle_fetch(result: Result<Vec<Schedule>, Box<MantleError>>) {
    let cb_struct = FETCH_CB_STRUCT.lock().unwrap();
    let result = to_java_result_list::<_, JavaSchedule>(result);
    RuntimeAndroid::exec_list(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_SchedulesKt_updateDeviceSchedule(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_schedule: JObject,
    j_callback: JObject,
) {
    let schedule = JavaSchedule::rust_object(MantleJObject(j_schedule), env).unwrap();
    let boxed_schedule = Box::into_raw(Box::new(schedule));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    UPDATE_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_update_device_schedule(cloudcore, boxed_schedule, handle_update)
}

fn handle_update(result: Result<Schedule, Box<MantleError>>) {
    let cb_struct = UPDATE_CB_STRUCT.lock().unwrap();
    let result = to_java_result::<_, JavaSchedule>(result);
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}