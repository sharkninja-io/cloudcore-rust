pub static JAVA_PACKAGE: &str = "com/sharkninja/cloudcore/";

mod cloudcore_ffi_api;

pub mod authentication;
pub mod cloudcore;
pub mod devices;
pub mod properties;
pub mod pairing;
pub mod cache;
pub mod account;
pub mod schedules;
pub mod notifications;

extern crate android_logger;
extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;