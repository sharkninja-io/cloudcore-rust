use cloudcore::CloudCore;
use std::os::raw::c_char;
use ffi_utilities::{convert_to_using_mantle_error, MantleStringPointer, RuntimeFFI};
use log::error;
use mantle_utilities::MantleError;
use cloudcore::authentication::UserSession;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_login(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    password: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let email = MantleStringPointer(email).to_option_string();
    let phone_number = MantleStringPointer(phone_number).to_option_string();
    let password = MantleStringPointer(password).to_string();
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.login(email, phone_number, password).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_get_user_session(
    ptr_cloudcore: *mut CloudCore,
) -> Result<UserSession, Box<MantleError>> {
    let cloudcore = &mut *ptr_cloudcore;
    convert_to_using_mantle_error(cloudcore.get_session())
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_set_user_session(
    ptr_cloudcore: *mut CloudCore,
    user_session: *const UserSession
) {
    let cloudcore = &mut *ptr_cloudcore;
    if !user_session.is_null() {
        let user_session = *Box::from_raw(user_session as *mut UserSession);
        cloudcore.set_session(user_session);
    } else {
        error!("Could not get pointer to UserSession to set it")
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_refresh_session(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.refresh_session().await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_logged_in(ptr_cloudcore: *mut CloudCore) -> bool {
    let cloudcore = &mut *ptr_cloudcore;
    cloudcore.logged_in()
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_logout(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let closure = async move {
        cloudcore.logout().await
    };
    RuntimeFFI::exec(closure, callback);
}
