#[cfg(feature = "library")]
use std::collections::HashMap;
#[cfg(feature = "library")]
use std::mem::size_of;
#[cfg(feature = "library")]
use std::ptr::null_mut;
#[cfg(feature = "library")]
use std::sync::Mutex;

#[cfg(feature = "library")]
use lazy_static::lazy_static;
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use mantle_utilities::{num_available_cpus, num_worker_threads};
#[cfg(feature = "library")]
use mantle_utilities::to_static_ref;
use serde::Serialize;

#[cfg(feature = "library")]
use crate::authentication::{CACHE_USER_DIR, CACHE_USER_SESSION_KEY};
use crate::authentication::UserSession;
#[cfg(feature = "library")]
use crate::cache::{CacheDataValue, CacheDir, CacheInteract};
use crate::cache::Cache;
#[cfg(feature = "library")]
use crate::cloudcore::AylaRegionEnvironment::{CNProd, CNDev, EUProd, NADev, NAProd};

#[derive(Serialize, Debug, Clone)]
pub struct ApplicationInfo {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone)]
pub struct SessionParameters {
    pub app_info: ApplicationInfo,
    pub user_url: String,
    pub device_url: String,
}

#[derive(Debug)]
pub struct CloudCore {
    pub user_session: Option<UserSession>,
    #[cfg(feature = "library")]
    pub selected_ayla_region_environment: AylaRegionEnvironment,
    #[cfg(feature = "library")]
    pub ayla_region_environment_map: HashMap<AylaRegionEnvironment, SessionParameters>,
    pub cache: Cache,
    #[cfg(feature = "library")]
    client: Option<reqwest::Client>,
    #[cfg(feature = "library")]
    blocking_client: Option<reqwest::blocking::Client>
}

#[cfg(feature = "library")]
pub static SELECTED_REGION_CACHE_KEY: &str = "countryRegionSelection";
#[cfg(feature = "library")]
pub static CACHE_LOGGING_DIR: &str = "logging";
#[cfg(feature = "library")]
pub static PAIRING_LOGGING_KEY: &str = "pairing";
pub static CACHE_APP_DIR: &str = "app";

#[cfg(feature = "library")]
lazy_static! {
    pub static ref SHARED: Mutex<usize> = Mutex::new(0);
}

#[cfg(feature = "library")]
impl CloudCore {
    pub fn new(os_dir: String) -> *mut CloudCore {
        let address_size = size_of::<usize>();
        debug!("On {} arch. Size of memory address {} bytes, {} bits, version {}", std::env::consts::ARCH, address_size, address_size*8, CloudCore::get_pkg_version());
        debug!("Using {} threads out of {} available", num_worker_threads(), num_available_cpus());
        let cache_result = Cache::new(os_dir.clone());
        if let Some(err) = cache_result.as_ref().err() {
            error!("Could not create cache: {}", err.to_string());
            return null_mut();
        }
        let mut cache = cache_result.unwrap();
        if !cache.child_paths().contains_key(CACHE_USER_DIR) {
            match cache.make_dir_for_child(CACHE_USER_DIR) {
                Ok(_) => debug!("cache made for user data"),
                Err(err) => error!("Could not create user data cache: {}", err.to_string())
            }
        }
        if !cache.child_paths().contains_key(CACHE_APP_DIR) {
            match cache.make_dir_for_child(CACHE_APP_DIR) {
                Ok(_) => debug!("cache made for app data"),
                Err(err) => error!("Could not create app data cache: {}", err.to_string())
            }
        }
        // Keep this for now so it is deleted for existing users
        if cache.child_paths().contains_key(CACHE_LOGGING_DIR) {
            if let Some(parent_path) = cache.parent_path().to_path_buf().to_str() {
                let path = format!("{}/{}",parent_path, CACHE_LOGGING_DIR);
                let _ = cache.remove_dir_for_child(&path);
            }
        }
        let us = get_user_session(&cache);
        let mut cc = CloudCore {
            user_session: us.clone(),
            selected_ayla_region_environment: NAProd,
            ayla_region_environment_map: HashMap::new(),
            cache,
            client: None,
            blocking_client: None,
        };
        if let Some(us) = us.as_ref() {
            let _ = &cc.set_ayla_region_environment(us.use_dev());
        }
        cc.clear_log();
        cc.create_log();
        let boxed = Box::into_raw(Box::new(cc));
        let addr = boxed as usize;
        if let Some(mut lock) = SHARED.lock().ok() {
            *lock = addr;
        }
        boxed
    }

    pub fn shared() -> &'static mut CloudCore {
        let mutex_guard = SHARED.lock().unwrap();
        let cloudcore = to_static_ref::<CloudCore>(*mutex_guard);
        drop(mutex_guard);
        cloudcore
    }

    pub fn session_params(&self) -> &SessionParameters {
        // If this panics the Rust cloudcore creation files did not create the map properly
        if let Some(sess_params) = self.ayla_region_environment_map.get(&self.selected_ayla_region_environment) {
            sess_params
        } else {
            error!("Selected Ayla region: {:?} does not exist for app", &self.selected_ayla_region_environment);
            panic!("Selected Ayla region: {:?} does not exist for app", &self.selected_ayla_region_environment);
        }
    }

    pub fn set_ayla_region_environment(&mut self, use_dev: bool) {
        let country = match self.cache.get_value(CACHE_USER_DIR.to_string(), SELECTED_REGION_CACHE_KEY.to_string()).unwrap_or(CacheDataValue::NullValue) {
            CacheDataValue::StringValue(string) => Some(string),
            _ => None
        };
        let region_env = match country {
            Some(country) => {
                debug!("have country set: {}", country);
                match country.as_str() {
                    "AT" | "BE" | "BG" | "HR" | "CY" | "DK" | "EE" | "FI" | "FR" | "DE" |
                    "GR" | "HU" | "IE" | "IT" | "LV" | "LT" | "LU" | "MT" | "NL" | "PL" |
                    "PT" | "RO" | "SK" | "SI" | "ES" | "SE" | "GB" | "UK" | "CZ" | "NO" |
                    "LI" | "CH"//EU countries
                    => EUProd,
                    "CN" | "ZH"  //china
                    => CNProd,
                    "US" | "CA" | "JP" | _ => { //united states, canada, japan, country not in list
                        if use_dev {
                            debug!("Using NA dev environment");
                            NADev
                        } else {
                            NAProd
                        }
                    }
                }
            }
            None => NAProd
        };
        self.selected_ayla_region_environment = region_env;
    }

    // For now this can't be used because a user can input a US number for China region user
    #[allow(dead_code)]
    fn user_country_code(&self) -> String {
        match &self.selected_ayla_region_environment {
            CNProd | CNDev => "+86",
            _ => "+1"
        }.to_string()
    }
    pub fn client(&self) -> &reqwest::Client {
        if CloudCore::shared().client.is_none() {
            CloudCore::shared().client = Some(reqwest::Client::new());
        }
        CloudCore::shared().client.as_ref().unwrap()
    }
    pub fn blocking_client(&self) -> &reqwest::blocking::Client {
        if CloudCore::shared().blocking_client.is_none() {
            CloudCore::shared().blocking_client = Some(reqwest::blocking::Client::new());
        }
        CloudCore::shared().blocking_client.as_ref().unwrap()
    }
    pub fn set_client(&mut self, client: reqwest::Client) {
        self.client = Some(client);
    }
    pub fn set_blocking_client(&mut self, blocking_client: reqwest::blocking::Client) {
        self.blocking_client = Some(blocking_client);
    }
    pub fn get_pkg_version() -> String {
        String::from(env!("CARGO_PKG_VERSION"))
    }
}

#[cfg(feature = "library")]
fn get_user_session(cache: &Cache) -> Option<UserSession> {
    let cache_data = cache.get_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
    if let Ok(data) = cache_data {
        match data {
            CacheDataValue::ObjectValue(val) => {
                let us: Option<UserSession> = serde_json::from_value(val).unwrap_or_else(|err| {
                    error!("Error getting saved user session: {}", err);
                    panic!("Error getting saved user session: {}", err)
                });
                if us.is_some() {
                    debug!("Have cached user session");
                }
                us
            }
            CacheDataValue::NullValue => None,
            _ => {
                error!("User session not saved as CacheDataValue::ObjectValue");
                panic!("User session not saved as CacheDataValue::ObjectValue")
            }
        }
    } else {
        debug!("No cached user session: {}", cache_data.err().unwrap().to_string());
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AylaRegionEnvironment {
    NAProd,
    NADev,
    EUProd,
    CNProd,
    CNDev,
}

pub static NA_PROD_USER_URL: &str = "https://user-field-39a9391a.aylanetworks.com";
pub static NA_PROD_DEVICE_URL: &str = "https://ads-field-39a9391a.aylanetworks.com";

pub static NA_DEV_USER_URL: &str = "https://user-dev.aylanetworks.com";
pub static NA_DEV_DEVICE_URL: &str = "https://ads-dev.aylanetworks.com";

pub static EU_PROD_USER_URL: &str = "https://user-field-eu.aylanetworks.com";
pub static EU_PROD_DEVICE_URL: &str = "https://ads-eu.aylanetworks.com";

pub static CN_PROD_USER_URL: &str = "https://user-field.ayla.com.cn";
pub static CN_PROD_DEVICE_URL: &str = "https://ads-field.ayla.com.cn";

pub static CN_DEV_USER_URL: &str = "https://user-dev.ayla.com.cn";
pub static CN_DEV_DEVICE_URL: &str = "https://ads-dev.ayla.com.cn";
