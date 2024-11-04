mod user_session;

use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jstring::MantleJString;
use android_utilities::{AndroidResult, CallbackStruct, JObjectRustBridge, RuntimeAndroid, to_java_result};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, jobject};
use jni::JNIEnv;
use crate::authentication::user_session::JavaUserSession;
use crate::cloudcore_ffi_api::CLOUDCORE_API;
use lazy_static::lazy_static;
use std::sync::Mutex;
use android_utilities::jni_exts::jobject::MantleJObject;
use mantle_utilities::MantleError;
use cloudcore::authentication::UserSession;
use cloudcore::CloudCore;

lazy_static! {
    static ref LOGIN_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref REFRESH_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref LOGOUT_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_login(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_email: JString,
    j_phone_number: JString,
    j_password: JString,
    j_callback: JObject,
) {
    let email = MantleJString(j_email).to_char_ptr(env);
    let phone_number = MantleJString(j_phone_number).to_char_ptr(env);
    let password = MantleJString(j_password).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    LOGIN_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_login(cloudcore, email, phone_number, password, handle_login)
}

fn handle_login(result: Result<(), Box<MantleError>>) {
    let cb_struct = LOGIN_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_getUserSession(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
) -> jobject {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    let result: Result<UserSession, Box<MantleError>> = CLOUDCORE_API.cloudcore_get_user_session(cloudcore);
    let result = to_java_result::<_, JavaUserSession>(result);
    *AndroidResult(result).to_jobject_result(env)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_setUserSession(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_user_session: JObject,
) {
    let user_session = JavaUserSession::rust_object(MantleJObject(j_user_session), env).unwrap();
    let boxed_session = Box::into_raw(Box::new(user_session));
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    CLOUDCORE_API.cloudcore_set_user_session(cloudcore, boxed_session);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_refreshSession(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    REFRESH_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_refresh_session(cloudcore, handle_refresh)
}

fn handle_refresh(result: Result<(), Box<MantleError>>) {
    let cb_struct = REFRESH_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_loggedIn(
    _env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong
) -> jboolean {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    CLOUDCORE_API.cloudcore_logged_in(cloudcore) as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AuthenticationKt_logout(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    LOGOUT_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_logout(cloudcore, handle_logout)
}

fn handle_logout(result: Result<(), Box<MantleError>>) {
    let cb_struct = LOGOUT_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}