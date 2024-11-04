#[cfg(feature = "library")]
use crate::pairing::ayla_device::AylaDeviceInfo;
#[cfg(feature = "library")]
use crate::pairing::wifi_network::WifiNetwork;
#[cfg(feature = "library")]
use crate::pairing::wifi_state::WifiPairingState;
#[cfg(feature = "library")]
use mantle_utilities::RUNTIME;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use log::error;
#[cfg(feature = "library")]
use tokio::task::JoinHandle;
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "library")]
use chrono::offset::Utc;

#[cfg(feature = "library")]
pub struct WifiPairingFailureTracker {
    last_error: Option<String>,
    error_count: u32
}

#[cfg(feature = "library")]
impl WifiPairingFailureTracker {
    fn new() -> Self {
        Self {
            last_error: None,
            error_count: 0
        }
    }
    pub fn set_error(&mut self, error: String) {
        self.error_count += 1;
        self.last_error = Some(error);
    }
}

#[cfg(feature = "library")]
pub struct WifiPairing {
    state_callback: Option<Box<dyn Fn(WifiPairingState) + Sync + Send + 'static>>,
    wifi_networks_callback: Option<Box<dyn Fn(Vec<WifiNetwork>) + Sync + Send + 'static>>,
    //stop_wifi_ap_scan: Option<fn(fn(stopped: bool))>,
    result_callback: Option<Box<dyn Fn(Result<String, Box<dyn Error>>) + Sync + Send + 'static>>,
    state: WifiPairingState,
    ayla_device_info: AylaDeviceInfo,
    access_token: Option<String>,
    join_handle: Option<JoinHandle<()>>,
    error_tracker: WifiPairingFailureTracker
}

#[cfg(not(feature = "library"))]
pub struct WifiPairing {}

#[cfg(feature = "library")]
impl WifiPairing {
    pub fn new(device_url: String) -> Self {
        Self {
            state_callback: None,
            wifi_networks_callback: None,
            result_callback: None,
            state: WifiPairingState::Idle,
            ayla_device_info: AylaDeviceInfo::new(device_url),
            access_token: None,
            join_handle: None,
            error_tracker: WifiPairingFailureTracker::new()
        }
    }

    pub fn configure(
        &mut self,
        state_callback: Box<dyn Fn(WifiPairingState) + Sync + Send + 'static>,
        wifi_networks_callback: Box<dyn Fn(Vec<WifiNetwork>) + Sync + Send + 'static>,
        result_callback: Box<dyn Fn(Result<String, Box<dyn Error>>) + Sync + Send + 'static>,
        access_token: Option<String>,
    ) {
        self.state_callback = Some(state_callback);
        self.wifi_networks_callback = Some(wifi_networks_callback);
        self.result_callback = Some(result_callback);
        self.access_token = access_token;
    }

    pub fn set_state(&mut self, state: WifiPairingState) {
        if self.state != WifiPairingState::Done {
            self.state = state;
        }
        if let Some(callback) = &self.state_callback {
            callback(self.state.to_owned())
        } else {
            debug!("no state callback set for {:?}?!", &self.state);
        }
    }

    pub fn set_join_handle(&mut self, join_handle: JoinHandle<()>) {
        self.join_handle = Some(join_handle);
    }

    pub fn start(&'static mut self, ip_address: String) {
        WifiPairing::log(format!("Have gateway IP: {}", &ip_address));
        if self.access_token.is_none() {
            self.handle_error("No user session!".to_string());
            WifiPairing::log("No user session!".to_string());
        } else if self.state.clone() as u8 >= WifiPairingState::SendingWiFiCredentialsToDevice as u8 {
            WifiPairing::log("Pairing already in process".to_string());
            error!("Pairing already in process");
        } else {
            WifiPairing::log(format!("Setting gateway ip address to use: {}", &ip_address));
            self.ayla_device_info.set_ip_address(Some(ip_address));
            self.continue_pairing();
        }
    }

    /// This method is used to allow the process to continue where it left off.
    /// For example, if an error occurred.
    pub fn continue_pairing(&'static mut self) {
        let ptr_manager = self as *mut WifiPairing;
        let handle = RUNTIME.spawn(async move {
            match self.state() {
                WifiPairingState::Idle | WifiPairingState::FetchingDSN => self.start_fetching_dsn().await,
                WifiPairingState::DeviceScanningWifi => {
                    let dsn = self.ayla_device_info().dsn().unwrap().to_string();
                    self.handle_dsn(dsn).await
                }
                // If a user is getting networks or sending credentials, start over so there is no stale or bad data persisted
                WifiPairingState::GettingWifiNetworks
                | WifiPairingState::SendingWiFiCredentialsToDevice => {
                    self.set_state(WifiPairingState::GettingWifiNetworks);
                    self.get_wifi_networks().await
                }
                WifiPairingState::EndingAccessPointsScanning => self.stop_device_access_point().await,
                WifiPairingState::PollingUserInternetConnection => self.wait_for_user_wifi().await,
                WifiPairingState::HandshakingWithAyla => self.connect_device_to_ayla().await,
                WifiPairingState::PollingDeviceOnAyla => self.check_device_connected().await,
                WifiPairingState::Connected => {
                    self.handle_connection_success(self.ayla_device_info.dsn().unwrap().to_string())
                },
                WifiPairingState::Done => {}
            }
        });
        let manager = unsafe { &mut *ptr_manager };
        manager.set_join_handle(handle);
    }

    pub fn done_pairing(mut self) {
        self.state = WifiPairingState::Done;
        self.abort_runtime_task();
    }

    pub fn abort_runtime_task(&self) {
        if let Some(join_handle) = &self.join_handle {
            debug!("aborting worker task in state: {:?}", self.state());
            join_handle.abort();
        }
    }

    pub fn log(contents: String) {
        let cc = CloudCore::shared();
        let contents = format!("{}: {}", Utc::now(), contents);
        let _ = cc.write_to_pairing_log(contents);
    }

    /// Getters
    pub fn state_callback(&self) -> Option<&Box<dyn Fn(WifiPairingState) + Sync + Send + 'static>> {
        self.state_callback.as_ref()
    }

    pub fn wifi_networks_callback(
        &self,
    ) -> Option<&Box<dyn Fn(Vec<WifiNetwork>) + Sync + Send + 'static>> {
        self.wifi_networks_callback.as_ref()
    }
    /*pub fn stop_wifi_ap_scan(&self) -> Option<&Box<dyn Fn() + Sync + Send + 'static>> {
        self.stop_wifi_ap_scan.as_ref()
    }*/
    pub fn result_callback(
        &self,
    ) -> Option<&Box<dyn Fn(Result<String, Box<dyn Error>>) + Sync + Send + 'static>> {
        self.result_callback.as_ref()
    }
    pub fn state(&self) -> &WifiPairingState {
        &self.state
    }
    pub fn ayla_device_info(&mut self) -> &mut AylaDeviceInfo {
        &mut self.ayla_device_info
    }
    pub fn access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
    }
    pub fn join_handle(&self) -> Option<&JoinHandle<()>> {
        self.join_handle.as_ref()
    }
    pub fn error_tracker(&mut self) -> &mut WifiPairingFailureTracker {
        &mut self.error_tracker
    }
}
