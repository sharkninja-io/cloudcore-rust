use std::sync::Mutex;
use android_utilities::{RuntimeAndroid, to_java_result_list};
use android_utilities::CallbackStruct;
use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jstring::MantleJString;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jlong;
use lazy_static::lazy_static;
use mantle_utilities::MantleError;

use cloudcore::CloudCore;
use cloudcore::notifications::notifications::Notification;

use crate::cloudcore_ffi_api::CLOUDCORE_API;
use crate::notifications::notification::JavaNotification;

mod notification;

lazy_static! {
    static ref NOTIFICATIONS_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref NOTIFICATIONS_CACHED_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DEL_NOTIFICATION_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DEL_NOTIFICATIONS_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref READ_NOTIFICATIONS_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_NotificationsKt_fetchAllNotifications(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_from: JString,
    j_callback: JObject,
) {
    let from = MantleJString(j_from).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    NOTIFICATIONS_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_fetch_all_notifications(cloudcore, from, handle_notifications);
}

fn handle_notifications(result: Result<Vec<Notification>, Box<MantleError>>) {
    let result = to_java_result_list::<_, JavaNotification>(result);
    let cb_struct = NOTIFICATIONS_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec_list(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_NotificationsKt_getCachedNotifications(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    NOTIFICATIONS_CACHED_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_get_cached_notifications(cloudcore, handle_cached_notifications);
}

fn handle_cached_notifications(result: Result<Vec<Notification>, Box<MantleError>>) {
    let result = to_java_result_list::<_, JavaNotification>(result);
    let cb_struct = NOTIFICATIONS_CACHED_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec_list(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_NotificationsKt_deleteNotification(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_dsn: JString,
    j_id: JString,
    j_callback: JObject,
) {
    let dsn = MantleJString(j_dsn).to_char_ptr(env);
    let id = MantleJString(j_id).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEL_NOTIFICATIONS_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_delete_notification(cloudcore, dsn, id, handle_delete_notification);
}

fn handle_delete_notification(result: Result<(), Box<MantleError>>) {
    let cb_struct = DEL_NOTIFICATIONS_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_NotificationsKt_deleteAllNotifications(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_to: JString,
    j_callback: JObject,
) {
    let to = MantleJString(j_to).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DEL_NOTIFICATIONS_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_delete_all_notifications(cloudcore, to, handle_delete_notifications);
}

fn handle_delete_notifications(result: Result<(), Box<MantleError>>) {
    let cb_struct = DEL_NOTIFICATIONS_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_NotificationsKt_markAllNotificationsAsRead(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    READ_NOTIFICATIONS_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_mark_all_notifications_as_read(cloudcore, handle_mark_notifications_read);
}

fn handle_mark_notifications_read(result: Result<(), Box<MantleError>>) {
    let cb_struct = READ_NOTIFICATIONS_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}
