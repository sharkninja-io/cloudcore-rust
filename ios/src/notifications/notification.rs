use std::os::raw::c_char;
use ffi_utilities::{MantleString, RustCBridge};
use cloudcore::notifications::notifications::Notification;

#[repr(C)]
#[derive(Debug)]
pub struct iOSNotification {
    user_uuid: *const c_char,
    id: *const c_char,
    dsn: *const c_char,
    created_at: *const c_char,
    datapoint_created_at: *const c_char,
    read: bool,
    deleted: bool,
    notification_type: i32,
    notification_subtype: i32,
}

impl RustCBridge<Notification> for iOSNotification {
    fn new_c_object(rust_object: &Notification) -> Self {
        Self {
            user_uuid: MantleString(rust_object.user_uuid().to_owned()).to_ptr(),
            id: MantleString(rust_object.id().to_owned()).to_ptr(),
            dsn: MantleString(rust_object.dsn().to_owned()).to_ptr(),
            created_at: MantleString(rust_object.created_at().to_owned()).to_ptr(),
            datapoint_created_at: MantleString(rust_object.datapoint_created_at().to_owned()).to_ptr(),
            read: rust_object.read().to_owned(),
            deleted: rust_object.deleted().to_owned(),
            notification_type: rust_object.notification_type().to_owned(),
            notification_subtype: rust_object.notification_subtype().to_owned(),
        }
    }
}