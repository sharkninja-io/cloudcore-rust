use cloudcore::CloudCore;
use std::os::raw::c_char;
use ffi_utilities::MantleResult;
use ios_utilities::{CallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mantle_utilities::MantleError;

lazy_static! {
    static ref CREATE_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref CONFIRM_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref SEND_CONFIRMATION_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref DELETE_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref REQUEST_PW_RESET_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref RESET_PW_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref RESET_USER_PW_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref UPDATE_EMAIL_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_create_account(
        ptr_cloudcore: *mut CloudCore,
        password: *const c_char,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_confirm_account(
        ptr_cloudcore: *mut CloudCore,
        token: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_send_confirmation_instructions(
        ptr_cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_delete_account(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_request_password_reset(
        ptr_cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_reset_password(
        ptr_cloudcore: *mut CloudCore,
        token: *const c_char,
        password: *const c_char,
        password_confirmation: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_reset_password_for_current_user(
        ptr_cloudcore: *mut CloudCore,
        current_password: *const c_char,
        new_password: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_update_email(
        ptr_cloudcore: *mut CloudCore,
        new_email: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_create_account(
    ptr_cloudcore: *mut CloudCore,
    password: *const c_char,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    CREATE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_create_account(ptr_cloudcore, password, email, phone_number, email_template_id, email_subject, email_body_html, handle_create);
}

fn handle_create(result: Result<(), Box<MantleError>>) {
    CREATE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_confirm_account(
    ptr_cloudcore: *mut CloudCore,
    token: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    CONFIRM_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_confirm_account(ptr_cloudcore, token, handle_confirm);
}

fn handle_confirm(result: Result<(), Box<MantleError>>) {
    CONFIRM_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_send_confirmation_instructions(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    SEND_CONFIRMATION_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_send_confirmation_instructions(ptr_cloudcore, email, phone_number, email_template_id, email_subject, email_body_html, handle_send_confirmation);
}

fn handle_send_confirmation(result: Result<(), Box<MantleError>>) {
    SEND_CONFIRMATION_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_delete_account(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    DELETE_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_delete_account(ptr_cloudcore, handle_delete);
}

fn handle_delete(result: Result<(), Box<MantleError>>) {
    DELETE_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_request_password_reset(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    REQUEST_PW_RESET_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_request_password_reset(ptr_cloudcore, email, phone_number, email_template_id, email_subject, email_body_html, handle_request_pw_reset);
}

fn handle_request_pw_reset(result: Result<(), Box<MantleError>>) {
    REQUEST_PW_RESET_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_reset_password(
    ptr_cloudcore: *mut CloudCore,
    token: *const c_char,
    password: *const c_char,
    password_confirmation: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    RESET_PW_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_reset_password(ptr_cloudcore, token, password, password_confirmation, handle_pw_reset);
}

fn handle_pw_reset(result: Result<(), Box<MantleError>>) {
    RESET_PW_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_reset_password_for_current_user(
    ptr_cloudcore: *mut CloudCore,
    current_password: *const c_char,
    new_password: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    RESET_USER_PW_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_reset_password_for_current_user(ptr_cloudcore, current_password, new_password, handle_pw_reset_for_current_user);
}

fn handle_pw_reset_for_current_user(result: Result<(), Box<MantleError>>) {
    RESET_USER_PW_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_update_email(
    ptr_cloudcore: *mut CloudCore,
    new_email: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    UPDATE_EMAIL_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_update_email(ptr_cloudcore, new_email, handle_update_email);
}

fn handle_update_email(result: Result<(), Box<MantleError>>) {
    UPDATE_EMAIL_CB_STRUCT.lock().unwrap().run(result);
}
