#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::fs::OpenOptions;
#[cfg(feature = "library")]
use std::io::Write;
#[cfg(feature = "library")]
use std::path::Path;
#[cfg(feature = "library")]
use bytes::Bytes;
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "library")]
use crate::io::{read_from_disk_to_string, write_to_disk};
#[cfg(feature = "library")]
use crate::pairing::wifi_network::WifiNetwork;
#[cfg(feature = "library")]
use crate::pairing::wifi_pairing::WifiPairing;
#[cfg(feature = "library")]
use crate::pairing::wifi_state::WifiPairingState;

mod ayla_device;

#[cfg(feature = "library")]
mod network_requests;

#[cfg(feature = "library")]
pub mod wifi_manager;

#[cfg(feature = "library")]
use lazy_static::lazy_static;
#[cfg(feature = "library")]
use std::sync::Mutex;

pub mod wifi_pairing;
pub mod wifi_network;
pub mod wifi_state;

#[cfg(feature = "library")]
lazy_static! {
    static ref PAIRING_LOCK: Mutex<usize> = Mutex::new(0);
}

#[cfg(feature = "library")]
impl CloudCore {
    pub fn create_pairing_manager(
        &self,
        state_callback: Box<dyn Fn(WifiPairingState) + Sync + Send + 'static>,
        get_wifi_networks_callback: Box<dyn Fn(Vec<WifiNetwork>) + Sync + Send + 'static>,
        result_callback: Box<dyn Fn(Result<String, Box<dyn Error>>) + Sync + Send + 'static>,
    ) -> WifiPairing {
        let mut manager = WifiPairing::new(self.session_params().device_url.to_owned());
        let token = match self.user_session.as_ref() {
            None => None,
            Some(session) => {
                if self.logged_in() {
                    Some(session.access_token().to_string())
                } else {
                    None
                }
            }
        };
        manager.configure(
            state_callback,
            get_wifi_networks_callback,
            result_callback,
            token
        );
        manager
    }
    pub fn write_to_pairing_log(&self, content: String) -> Result<(), Box<dyn Error>> {
        match PAIRING_LOCK.lock() {
            Ok(lock) => {
                let path = self.cache.parent_path().join(Path::new("pairing_logging"));
                if let Some(path_str) = path.to_str() {
                    match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(path_str) {
                            Ok(mut file ) => {
                                let content = format!("{}\n", content);
                                if let Err(e) = file.write_all(& *Bytes::from(content.into_bytes())) {
                                    drop(lock);
                                    Err(e.into())
                                } else {
                                    drop(lock);
                                    Ok(())
                                }
                            }
                        Err(err) => {
                            Err(err.into())
                        }
                    }
                } else {
                    drop(lock);
                    Err("Failed to get path as str".into())
                }
            }
            Err(err) => {
                error!("Failed to get lock to write pairing log. Maybe need to call this from another thread");
                Err(err.into())
            }
        }
    }
    pub fn create_log(&self) {
        let path = self.cache.parent_path().join(Path::new("pairing_logging"));
        if !path.exists() {
            debug!("creating pairing log");
            if let Some(error) = write_to_disk(path.as_path(), Bytes::from(String::new().into_bytes())).err() {
                error!("Failed to create pairing log {}", error.to_string());
            }
        }
    }
    pub fn clear_log(&self) {
        let path = self.cache.parent_path().join(Path::new("pairing_logging"));
        if path.exists() {
            if let Some(error) = write_to_disk(path.as_path(), Bytes::from(String::new().into_bytes())).err() {
                error!("Failed to clear pairing log {}", error.to_string());
            }
        }
    }
    pub fn get_pairing_log(&self) -> Result<String, Box<dyn Error>> {
        let path = self.cache.parent_path().join(Path::new("pairing_logging"));
        if let Some(path_str) = path.to_str() {
            read_from_disk_to_string(path_str)
        } else {
            Err("Failed to get path as str".into())
        }
    }
}