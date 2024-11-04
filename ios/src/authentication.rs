mod user_session;

use std::os::raw::c_char;
use ffi_utilities::{CRustBridge, MantleResult, to_c_object};
use ios_utilities::{CallbackStruct};
use lazy_static::lazy_static;
use std::sync::Mutex;
use log::error;
use mantle_utilities::MantleError;
use cloudcore::authentication::UserSession;
use crate::authentication::user_session::UserSession as iOSUserSession;
use cloudcore::CloudCore;

lazy_static! {
    static ref LOGIN_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref REFRESH_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
    static ref LOGOUT_CB_STRUCT: Mutex<CallbackStruct<()>> = Mutex::new(CallbackStruct::new());
}

#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn cloudcore_login(
        ptr_cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        password: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_get_user_session(
        ptr_cloudcore: *mut CloudCore,
    ) -> Result<UserSession, Box<MantleError>>;
    fn cloudcore_set_user_session(
        ptr_cloudcore: *mut CloudCore,
        user_session: *const UserSession
    );
    fn cloudcore_refresh_session(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
    fn cloudcore_logged_in(cloudcore: *mut CloudCore) -> bool;
    fn cloudcore_logout(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_login(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    password: *const c_char,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    LOGIN_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_login(ptr_cloudcore, email, phone_number, password, handle_login);
}

fn handle_login(result: Result<(), Box<MantleError>>) {
    LOGIN_CB_STRUCT.lock().unwrap().run(result);
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_get_user_session(
    ptr_cloudcore: *mut CloudCore,
) -> MantleResult<iOSUserSession> {
    to_c_object(cloudcore_get_user_session(ptr_cloudcore))
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_set_user_session(
    ptr_cloudcore: *mut CloudCore,
    ptr_user_session: *const iOSUserSession
) {
    if let Some(user_session) = iOSUserSession::new_rust_object(ptr_user_session) {
        let user_session = Box::into_raw(Box::new(user_session));
        cloudcore_set_user_session(ptr_cloudcore, user_session);
    } else {
        error!("Could not get pointer to UserSession to set it")
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_refresh_session(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    REFRESH_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_refresh_session(ptr_cloudcore, handle_refresh);
}

fn handle_refresh(result: Result<(), Box<MantleError>>) {
    REFRESH_CB_STRUCT.lock().unwrap().run(result);
}


// Note: This does not need to exist. Callers could just call cloudcore_logged_in, but it is here for consistency
#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_logged_in(cloudcore: *mut CloudCore) -> bool {
    cloudcore_logged_in(cloudcore)
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn ios_cloudcore_logout(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: MantleResult<()>, callback_id: u64),
    callback_id: u64,
) {
    LOGOUT_CB_STRUCT.lock().unwrap().update(callback, callback_id);
    cloudcore_logout(ptr_cloudcore, handle_logout);
}

fn handle_logout(result: Result<(), Box<MantleError>>) {
    LOGOUT_CB_STRUCT.lock().unwrap().run(result);
}

