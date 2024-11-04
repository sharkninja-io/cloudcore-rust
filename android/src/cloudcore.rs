use std::collections::HashMap;
use android_logger::Config;
use android_utilities::java_class_names;
use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jstring::MantleJString;
use jni::objects::{JClass, JString};
use jni::JNIEnv;
use jni::sys::jlong;
use log::Level;

use cloudcore::CloudCore;
use cloudcore::cloudcore::{ApplicationInfo, AylaRegionEnvironment};
use crate::cloudcore_ffi_api::CLOUDCORE_API;

use libc::{c_int, sighandler_t, signal};

#[no_mangle]
pub unsafe extern "C" fn bsd_signal(signum: c_int, handler: sighandler_t) -> sighandler_t {
    signal(signum, handler)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_CloudCoreSDK_create(
    env: JNIEnv,
    _class: JClass,
    j_os_dir: JString,
) -> *const CloudCore {
    android_logger::init_once(Config::default().with_min_level(Level::Debug));
    let region_map = create_ayla_region_environment_map();
    let region_map = Box::into_raw(Box::new(region_map));
    let os_dir = MantleJString(j_os_dir).to_char_ptr(env);
    let cloudcore = CLOUDCORE_API.cloudcore_create(os_dir, region_map);
    java_class_names::capture_class_refs(env);
    cloudcore
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_CloudCoreSDK_destroy(
    _env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
) {
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    CLOUDCORE_API.cloudcore_destroy(cloudcore)
}

fn create_ayla_region_environment_map() -> HashMap<AylaRegionEnvironment, ApplicationInfo> {
    let mut map: HashMap<AylaRegionEnvironment, ApplicationInfo> = HashMap::new();

    map.insert(AylaRegionEnvironment::NAProd, ApplicationInfo {
        app_id: "Shark-Android-field-id".to_string(),
        app_secret: "Shark-Android-field-Wv43MbdXRM297HUHotqe6lU1n-w".to_string()
    });

    map.insert(AylaRegionEnvironment::NADev, ApplicationInfo {
        app_id: "Vac-Android-dev-id".to_string(),
        app_secret: "Vac-Android-dev-x2vTY_FX3pt72LQGkPzErjabrkg".to_string()
    });

    map.insert(AylaRegionEnvironment::EUProd, ApplicationInfo {
        app_id: "Shark-Android-EUField-Fw-id".to_string(),
        app_secret: "Shark-Android-EUField-s-zTykblGJujGcSSTaJaeE4PESI".to_string()
    });

    map.insert(AylaRegionEnvironment::CNProd, ApplicationInfo {
        app_id: "Shark-Android-CNField-9Q-id".to_string(),
        app_secret: "Shark-Android-CNField-CIXRYqibaPNKxALEEGQPlJ00B20".to_string()
    });

    map
}
