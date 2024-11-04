use std::os::raw::c_char;
use ffi_utilities::{CRustBridge, MantleString, MantleStringPointer, RustCBridge};

#[repr(C)]
#[derive(Debug)]
pub struct UserSession {
    access_token: *const c_char,
    refresh_token: *const c_char,
    auth_expiration_date: u64,
    auth_username: *const c_char,
    user_uuid: *const c_char,
    use_dev: bool,
}

impl RustCBridge<cloudcore::authentication::UserSession> for UserSession {
    fn new_c_object(rust_object: &cloudcore::authentication::UserSession) -> Self {
        Self {
            access_token: MantleString(rust_object.access_token().to_string()).to_ptr(),
            refresh_token: MantleString(rust_object.refresh_token().to_string()).to_ptr(),
            auth_expiration_date: rust_object.auth_expiration_date().to_owned(),
            auth_username: MantleString(rust_object.auth_username().to_string()).to_ptr(),
            user_uuid: MantleString(rust_object.user_uuid().unwrap().to_owned()).to_ptr(),
            use_dev: rust_object.use_dev().to_owned()
        }
    }
}

// This is needed so the first time an app switches to using CC for auth it can pass in it's saved auth params
impl CRustBridge<cloudcore::authentication::UserSession> for UserSession {
    unsafe fn new_rust_object(
        c_object_ptr: *const Self,
    ) -> Option<cloudcore::authentication::UserSession> {
        if c_object_ptr.is_null() {
            Option::None
        } else {
            match c_object_ptr.as_ref() {
                None => Option::None,
                Some(obj_ref) => Some(cloudcore::authentication::UserSession::new(
                    MantleStringPointer(obj_ref.access_token).to_string(),
                    MantleStringPointer(obj_ref.refresh_token).to_string(),
                    obj_ref.auth_expiration_date.to_owned(),
                    MantleStringPointer(obj_ref.auth_username).to_string(),
                    Some(MantleStringPointer(obj_ref.user_uuid).to_string()),
                    obj_ref.use_dev.to_owned()
                )),
            }
        }
    }
}