use std::collections::HashMap;
use std::os::raw::c_char;

#[cfg(target_os = "android")]
use android_logger::Config as AndroidConfig;
use ffi_utilities::{MantleStringPointer, MantleString};
#[cfg(not(target_os = "android"))]
use log::LevelFilter;
#[cfg(target_os = "android")]
use log::Level;
#[cfg(not(target_os = "android"))]
use simplelog::{Config, SimpleLogger};

use log::error;

use cloudcore::CloudCore;
use cloudcore::cloudcore::{ApplicationInfo, AylaRegionEnvironment, CN_DEV_DEVICE_URL, CN_DEV_USER_URL, CN_PROD_DEVICE_URL, CN_PROD_USER_URL, EU_PROD_DEVICE_URL, EU_PROD_USER_URL, NA_DEV_DEVICE_URL, NA_DEV_USER_URL, NA_PROD_DEVICE_URL, NA_PROD_USER_URL, SessionParameters};
use cloudcore::cloudcore::AylaRegionEnvironment::{CNDev, CNProd, EUProd, NADev, NAProd};

#[cfg(not(target_os = "android"))]
fn load_logger() {
    if let Some(err) = SimpleLogger::init(LevelFilter::Debug, Config::default()).err() {
        println!("Error setting logger: {}", err.to_string())
    }
}

#[cfg(target_os = "android")]
fn load_logger() {
    android_logger::init_once(AndroidConfig::default().with_min_level(Level::Debug));
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_create(
    os_dir: *const c_char,
    hash_map: *mut HashMap<AylaRegionEnvironment, ApplicationInfo>,
) -> *const CloudCore {
    let for_debug = cfg!(debug_assertions);
    if for_debug {
        load_logger();
    }
    let os_dir = MantleStringPointer(os_dir).to_string();
    let hash_map = *Box::from_raw(hash_map);
    let mut cloudcore = CloudCore::new(os_dir);
    (*cloudcore).ayla_region_environment_map = create_ayla_region_environment_map(hash_map);
    cloudcore
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_destroy(
    ptr_cloudcore: *mut CloudCore,
) {
    if ptr_cloudcore.is_null() {
        error!("cloudcore object pointer is null");
        return;
    }
    let _ = *Box::from_raw(ptr_cloudcore);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
    pub unsafe extern "C" fn cloudcore_app_id(
    ptr_cloudcore: *mut CloudCore,
) -> *const c_char {
    let cloudcore = &mut *ptr_cloudcore;
    let app_id = cloudcore.session_params().app_info.app_id.clone();
    return MantleString(app_id.to_owned()).to_ptr()
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn cloudcore_selected_ayla_region(
    ptr_cloudcore: *mut CloudCore,
) -> *mut AylaRegionEnvironment {
    let cloudcore = &mut *ptr_cloudcore;
    Box::into_raw(Box::new(cloudcore.selected_ayla_region_environment.clone()))
}

fn create_ayla_region_environment_map(app_map: HashMap<AylaRegionEnvironment, ApplicationInfo>) -> HashMap<AylaRegionEnvironment, SessionParameters> {
    let mut map: HashMap<AylaRegionEnvironment, SessionParameters> = HashMap::new();

    if let Some(app_info) = app_map.get(&NAProd)
    {
        let na_prod = SessionParameters {
            app_info: app_info.clone(),
            user_url: NA_PROD_USER_URL.to_string(),
            device_url: NA_PROD_DEVICE_URL.to_string(),
        };
        map.insert(AylaRegionEnvironment::NAProd, na_prod);
    }

    if let Some(app_info) = app_map.get(&NADev)
    {
        let na_dev = SessionParameters {
            app_info: app_info.clone(),
            user_url: NA_DEV_USER_URL.to_string(),
            device_url: NA_DEV_DEVICE_URL.to_string(),
        };
        map.insert(AylaRegionEnvironment::NADev, na_dev);
    }

    if let Some(app_info) = app_map.get(&EUProd)
    {
        let eu_prod = SessionParameters {
            app_info: app_info.clone(),
            user_url: EU_PROD_USER_URL.to_string(),
            device_url: EU_PROD_DEVICE_URL.to_string(),
        };
        map.insert(AylaRegionEnvironment::EUProd, eu_prod);
    }

    if let Some(app_info) = app_map.get(&CNProd)
    {
        let cn_prod = SessionParameters {
            app_info: app_info.clone(),
            user_url: CN_PROD_USER_URL.to_string(),
            device_url: CN_PROD_DEVICE_URL.to_string(),
        };
        map.insert(AylaRegionEnvironment::CNProd, cn_prod);
    }

    if let Some(app_info) = app_map.get(&CNDev)
    {
        let cn_dev = SessionParameters {
            app_info: app_info.clone(),
            user_url: CN_DEV_USER_URL.to_string(),
            device_url: CN_DEV_DEVICE_URL.to_string(),
        };
        map.insert(AylaRegionEnvironment::CNDev, cn_dev);
    }
    map
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_get_pkg_version() -> *mut String {
    Box::into_raw(Box::new(CloudCore::get_pkg_version()))
}