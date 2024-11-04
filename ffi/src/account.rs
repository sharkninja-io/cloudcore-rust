use cloudcore::CloudCore;
use std::os::raw::c_char;
use ffi_utilities::{MantleStringPointer, RuntimeFFI};
use mantle_utilities::MantleError;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_create_account(
    ptr_cloudcore: *mut CloudCore,
    password: *const c_char,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let password = MantleStringPointer(password).to_string();
    let email = MantleStringPointer(email).to_option_string();
    let phone_number = MantleStringPointer(phone_number).to_option_string();
    let email_template_id = MantleStringPointer(email_template_id).to_option_string();
    let email_subject = MantleStringPointer(email_subject).to_option_string();
    let email_body_html = MantleStringPointer(email_body_html).to_option_string();

    let closure = async move {
        cloudcore
            .create_account(
                password,
                email,
                phone_number,
                email_template_id,
                email_subject,
                email_body_html,
            )
            .await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_confirm_account(
    ptr_cloudcore: *mut CloudCore,
    token: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let token = MantleStringPointer(token).to_string();

    let closure = async move {
        cloudcore.confirm_account(token).await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_send_confirmation_instructions(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let email = MantleStringPointer(email).to_option_string();
    let phone_number = MantleStringPointer(phone_number).to_option_string();
    let email_template_id = MantleStringPointer(email_template_id).to_option_string();
    let email_subject = MantleStringPointer(email_subject).to_option_string();
    let email_body_html = MantleStringPointer(email_body_html).to_option_string();

    let closure = async move {
        cloudcore
            .send_confirmation_instructions(
                email,
                phone_number,
                email_template_id,
                email_subject,
                email_body_html,
            )
            .await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_delete_account(
    ptr_cloudcore: *mut CloudCore,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;

    let closure = async move {
        cloudcore.delete_account().await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_request_password_reset(
    ptr_cloudcore: *mut CloudCore,
    email: *const c_char,
    phone_number: *const c_char,
    email_template_id: *const c_char,
    email_subject: *const c_char,
    email_body_html: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let email = MantleStringPointer(email).to_option_string();
    let phone_number = MantleStringPointer(phone_number).to_option_string();
    let email_template_id = MantleStringPointer(email_template_id).to_option_string();
    let email_subject = MantleStringPointer(email_subject).to_option_string();
    let email_body_html = MantleStringPointer(email_body_html).to_option_string();

    let closure = async move {
        cloudcore
            .request_password_reset(email, phone_number, email_template_id, email_subject, email_body_html)
            .await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_reset_password(
    ptr_cloudcore: *mut CloudCore,
    token: *const c_char,
    password: *const c_char,
    password_confirmation: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let token = MantleStringPointer(token).to_string();
    let password = MantleStringPointer(password).to_string();
    let password_confirmation = MantleStringPointer(password_confirmation).to_string();

    let closure = async move {
        cloudcore
            .reset_password(token, password, password_confirmation)
            .await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_reset_password_for_current_user(
    ptr_cloudcore: *mut CloudCore,
    current_password: *const c_char,
    new_password: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let current_password = MantleStringPointer(current_password).to_string();
    let new_password = MantleStringPointer(new_password).to_string();

    let closure = async move {
        cloudcore
            .reset_password_for_current_user(current_password, new_password)
            .await
    };
    RuntimeFFI::exec(closure, callback);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_update_email(
    ptr_cloudcore: *mut CloudCore,
    new_email: *const c_char,
    callback: fn(result: Result<(), Box<MantleError>>),
) {
    let cloudcore = &mut *ptr_cloudcore;
    let new_email = MantleStringPointer(new_email).to_string();

    let closure = async move {
        cloudcore.update_email(new_email).await
    };
    RuntimeFFI::exec(closure, callback);
}
