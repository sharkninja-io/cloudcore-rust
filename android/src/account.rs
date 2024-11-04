use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jstring::MantleJString;
use android_utilities::{RuntimeAndroid, CallbackStruct};
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use jni::sys::jlong;
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;
use cloudcore::CloudCore;
use crate::cloudcore_ffi_api::CLOUDCORE_API;

lazy_static! {
    static ref CREATE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref CONFIRM_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref SEND_CONFIRMATION_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref DELETE_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref REQUEST_PW_RESET_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref RESET_PW_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref RESET_USER_PW_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref UPDATE_EMAIL_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_createAccount(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_password: JString,
    j_email: JString,
    j_phone_number: JString,
    j_email_template_id: JString,
    j_email_subject: JString,
    j_email_body_html: JString,
    j_callback: JObject,
) {
    let email = MantleJString(j_email).to_char_ptr(env);
    let phone_number = MantleJString(j_phone_number).to_char_ptr(env);
    let password = MantleJString(j_password).to_char_ptr(env);
    let email_template_id = MantleJString(j_email_template_id).to_char_ptr(env);
    let email_subject = MantleJString(j_email_subject).to_char_ptr(env);
    let email_body_html = MantleJString(j_email_body_html).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    CREATE_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_create_account(cloudcore, password, email, phone_number, email_template_id, email_subject, email_body_html, handle_create);
}

fn handle_create(result: Result<(), Box<MantleError>>) {
    let cb_struct = CREATE_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_confirmAccount(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_token: JString,
    j_callback: JObject,
) {
    let token = MantleJString(j_token).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    CONFIRM_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_confirm_account(cloudcore, token, handle_confirm);
}

fn handle_confirm(result: Result<(), Box<MantleError>>) {
    let cb_struct = CONFIRM_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_sendConfirmation(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_email: JString,
    j_phone_number: JString,
    j_email_template_id: JString,
    j_email_subject: JString,
    j_email_body_html: JString,
    j_callback: JObject,
) {
    let email = MantleJString(j_email).to_char_ptr(env);
    let phone_number = MantleJString(j_phone_number).to_char_ptr(env);
    let email_template_id = MantleJString(j_email_template_id).to_char_ptr(env);
    let email_subject = MantleJString(j_email_subject).to_char_ptr(env);
    let email_body_html = MantleJString(j_email_body_html).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    SEND_CONFIRMATION_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_send_confirmation_instructions(cloudcore, email, phone_number, email_template_id, email_subject, email_body_html, handle_send_confirmation)
}

fn handle_send_confirmation(result: Result<(), Box<MantleError>>) {
    let cb_struct = SEND_CONFIRMATION_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_deleteAccount(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_callback: JObject,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    DELETE_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_delete_account(cloudcore, handle_delete);
}

fn handle_delete(result: Result<(), Box<MantleError>>) {
    let cb_struct = DELETE_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_requestPasswordReset(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore:jlong,
    j_email: JString,
    j_phone_number: JString,
    j_email_template_id: JString,
    j_email_subject: JString,
    j_email_body_html: JString,
    j_callback: JObject,
) {
    let email = MantleJString(j_email).to_char_ptr(env);
    let phone_number = MantleJString(j_phone_number).to_char_ptr(env);
    let email_template_id = MantleJString(j_email_template_id).to_char_ptr(env);
    let email_subject = MantleJString(j_email_subject).to_char_ptr(env);
    let email_body_html = MantleJString(j_email_body_html).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    REQUEST_PW_RESET_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_request_password_reset(cloudcore, email, phone_number, email_template_id, email_subject, email_body_html, handle_request_pw_reset);
}

fn handle_request_pw_reset(result: Result<(), Box<MantleError>>) {
    let cb_struct = REQUEST_PW_RESET_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_resetPassword(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_token: JString,
    j_password: JString,
    j_password_confirmation: JString,
    j_callback: JObject,
) {
    let token = MantleJString(j_token).to_char_ptr(env);
    let password = MantleJString(j_password).to_char_ptr(env);
    let password_confirmation = MantleJString(j_password_confirmation).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    RESET_PW_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_reset_password(cloudcore, token, password, password_confirmation, handle_pw_reset);
}

fn handle_pw_reset(result: Result<(), Box<MantleError>>) {
    let cb_struct = RESET_PW_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_resetUserPassword(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_current_password: JString,
    j_new_password: JString,
    j_callback: JObject,
) {
    let current_password = MantleJString(j_current_password).to_char_ptr(env);
    let new_password = MantleJString(j_new_password).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    RESET_USER_PW_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_reset_password_for_current_user(cloudcore, current_password, new_password, handle_pw_reset_for_current_user);
}

fn handle_pw_reset_for_current_user(result: Result<(), Box<MantleError>>) {
    let cb_struct = RESET_USER_PW_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_AccountKt_updateEmail(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_new_email: JString,
    j_callback: JObject,
) {
    let new_email = MantleJString(j_new_email).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    UPDATE_EMAIL_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_update_email(cloudcore, new_email, handle_update_email);
}

fn handle_update_email(result: Result<(), Box<MantleError>>) {
    let cb_struct = UPDATE_EMAIL_CB_STRUCT.lock().unwrap();
    RuntimeAndroid::exec(&cb_struct.jvm, result, &cb_struct.callback);
}
