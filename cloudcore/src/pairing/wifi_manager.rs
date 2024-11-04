use crate::pairing::network_requests::{
    ayla_device_handshake, fetch_dsn, fetch_wifi_networks, get_device,
    send_wifi_credentials_to_device, start_wifi_scan, stop_device_access_point,
};
use crate::pairing::wifi_network::WifiNetwork;
use crate::pairing::wifi_pairing::WifiPairing;
use crate::pairing::wifi_state::WifiPairingState;
use async_recursion::async_recursion;
use log::{debug, error};
use mantle_utilities::RUNTIME;
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use crate::CloudCore;

static MAX_FAILS: u32 = 20;
static QUICK_PAUSE_SECS: u64 = 3;
static PAUSE_SECS: u64 = 5;
static AYLA_CONNECTION_CHECK_PAUSE_SECS: u64 = 10; // RTH 2022-07-20: This is to match gen 1 retry behavior
static AYLA_CONNECTION_CHECK_MAX_FAILS: u32 = 20; // RTH 2022-07-20: This is to match gen 1 retry behavior

pub fn handle_wifi_network(manager: &'static mut WifiPairing, wifi_network: WifiNetwork) {
    if manager.state().clone() as u8 >= WifiPairingState::SendingWiFiCredentialsToDevice as u8 {
        error!("Already sent wifi credentials to device");
        WifiPairing::log("Already sent wifi credentials to device".to_string());
        return;
    }
    let setup_token = random_ayla_token();
    manager
        .ayla_device_info()
        .set_setup_token(Some(setup_token.to_owned()));
    manager.set_state(WifiPairingState::SendingWiFiCredentialsToDevice);
    let ip_address = manager.ayla_device_info().ip_address().unwrap().to_owned();
    let ptr_manager = manager as *mut WifiPairing;
    let handle = RUNTIME.spawn(async move {
        let mut fail_count = 0;
        let mut result: Result<(), Box<dyn Error + Send + Sync>> = Err("".into());
        while fail_count < MAX_FAILS && result.is_err() {
            debug!("sending credentials to device to join selected SSID");
            WifiPairing::log("sending credentials to device to join selected SSID".to_string());
            result = send_wifi_credentials_to_device(
                ip_address.to_owned(),
                wifi_network.clone(),
                setup_token.to_owned(),
            )
            .await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < MAX_FAILS {
                sleep(Duration::from_secs(QUICK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(_) => {
                debug!("🎉 connected device IP {} to wifi network {}", &ip_address, &wifi_network.ssid().unwrap());
                WifiPairing::log(format!("🎉 connected device IP {} to wifi network {}", &ip_address, &wifi_network.ssid().unwrap()));
                manager.stop_device_access_point().await
            }
            Err(err) => manager.handle_error(err.to_string()),
        }
    });
    let manager = unsafe { &mut *ptr_manager };
    manager.set_join_handle(handle);
}

impl WifiPairing {

    pub async fn start_fetching_dsn(&'static mut self) {
        self.set_state(WifiPairingState::FetchingDSN);
        let ip_address = self.ayla_device_info().ip_address().unwrap().to_owned();
        let mut fail_count = 0;
        let mut result: Result<String, Box<dyn Error + Send + Sync>> = Err("".into());
        while fail_count < MAX_FAILS && result.is_err() {
            debug!("fetching dsn for IP {}", &ip_address);
            WifiPairing::log(format!("fetching dsn for IP {}", &ip_address));
            result = fetch_dsn(ip_address.to_owned()).await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < MAX_FAILS {
                sleep(Duration::from_secs(QUICK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(dsn) => self.handle_dsn(dsn).await,
            Err(err) => self.handle_error(err.to_string()),
        };
    }

    pub async fn handle_dsn(&'static mut self, dsn: String) {
        debug!("Got dsn: {}", &dsn);
        WifiPairing::log(format!("Got dsn: {}", &dsn));
        self.ayla_device_info().set_dsn(Some(dsn));
        self.set_state(WifiPairingState::DeviceScanningWifi);
        let ip_address = self.ayla_device_info().ip_address().unwrap().to_owned();
        let mut fail_count = 0;
        let mut result: Result<(), Box<dyn Error + Send + Sync>> = Err("".into());
        while fail_count < MAX_FAILS && result.is_err() {
            debug!("starting wifi scan for IP {}", &ip_address);
            WifiPairing::log(format!("starting wifi scan for IP {}", &ip_address));
            result = start_wifi_scan(ip_address.to_owned()).await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < MAX_FAILS {
                sleep(Duration::from_secs(QUICK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(_) => self.get_wifi_networks().await,
            Err(err) => self.handle_error(err.to_string()),
        };
    }

    pub async fn get_wifi_networks(&'static mut self) {
        if self.state().clone() as u8 >= WifiPairingState::SendingWiFiCredentialsToDevice as u8 {
            return;
        }
        self.set_state(WifiPairingState::GettingWifiNetworks);
        self.run_get_wifi_networks_loop().await;
    }

    #[async_recursion]
    async fn run_get_wifi_networks_loop(&'static mut self) {
        if self.state().clone() as u8 >= WifiPairingState::SendingWiFiCredentialsToDevice as u8 {
            return;
        }
        let mut fail_count = 0;
        let mut result: Result<Vec<WifiNetwork>, Box<dyn Error + Send + Sync>> = Err("".into());
        let ip_address = self.ayla_device_info().ip_address().unwrap().to_owned();
        while fail_count < MAX_FAILS && result.is_err() {
            debug!("fetching visible wifi networks for IP {}", &ip_address);
            WifiPairing::log(format!("starting wifi scan for IP {}", &ip_address));
            result = fetch_wifi_networks(ip_address.to_owned()).await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < MAX_FAILS {
                sleep(Duration::from_secs(QUICK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(networks) => {
                debug!("wireless networks for IP {}: {:?}", &ip_address, &networks);
                WifiPairing::log(format!("wireless networks for IP {}: {:?}", &ip_address, &networks));
                self.get_desired_wifi_network(networks);
                sleep(Duration::from_secs(PAUSE_SECS)).await;
                self.run_get_wifi_networks_loop().await;
            }
            Err(err) => self.handle_error(err.to_string()),
        };
    }

    pub fn get_desired_wifi_network(&mut self, wifi_networks: Vec<WifiNetwork>) {
        if matches!(self.state(), WifiPairingState::GettingWifiNetworks) {
            match self.wifi_networks_callback() {
                Some(callback) => callback(wifi_networks),
                None => self.handle_error(
                    "Need a wifi network connected to the internet for the device".to_string(),
                ),
            }
        }
    }

    pub async fn stop_device_access_point(&'static mut self) {
        self.set_state(WifiPairingState::EndingAccessPointsScanning);
        let ip_address = self.ayla_device_info().ip_address().unwrap().to_owned();
        // For now do not get a join handle, since it should just run once
        RUNTIME.spawn(async move {
            // This may fail. May not. Doesn't negatively affect anything.
            debug!("Sending command to stop AP for IP {}", &ip_address);
            WifiPairing::log(format!("Sending command to stop AP for IP {}", &ip_address));
            let _ = stop_device_access_point(ip_address).await;
        });
        self.wait_for_user_wifi().await;
    }

    pub async fn wait_for_user_wifi(&'static mut self) {
        self.set_state(WifiPairingState::PollingUserInternetConnection);
        debug!("Starting wait to automatically rejoin known wifi with internet");
        sleep(Duration::from_secs(PAUSE_SECS)).await;
        /*WifiPairing::log(format!("Starting wait to automatically rejoin known wifi with internet"));
        let mut result: Result<(), Box<dyn Error + Send + Sync>> = Err("".into());
        while result.is_err() {
            debug!("trying internet connection...");
            WifiPairing::log(format!("trying internet connection..."));
            result = try_internet_request().await;
            if result.is_err() {
                debug!(
                    "Failed connecting to internet: {}",
                    result.as_ref().err().unwrap()
                );
                WifiPairing::log(format!("Failed connecting to internet: {}",
                                 result.as_ref().err().unwrap()));
                sleep(Duration::from_secs(PAUSE_SECS)).await
            }
        }
        debug!("connected back to internet");
        WifiPairing::log(format!("connected back to internet"));*/
        self.connect_device_to_ayla().await;
    }

    pub async fn connect_device_to_ayla(&'static mut self) {
        self.set_state(WifiPairingState::HandshakingWithAyla);
        let dsn = self.ayla_device_info().dsn().unwrap().to_owned();
        let setup_token = self.ayla_device_info().setup_token().unwrap().to_owned();
        let access_token = self.access_token().as_ref().unwrap().to_string();
        let device_url = self.ayla_device_info().device_url().to_owned();
        let mut fail_count = 0;
        let mut result: Result<(), Box<dyn Error + Send + Sync>> = Err("".into());
        while fail_count < MAX_FAILS && result.is_err() {
            debug!("connecting device to ayla");
            WifiPairing::log("connecting device to ayla".to_string());
            result = ayla_device_handshake(
                device_url.to_string(),
                dsn.to_string(),
                access_token.to_string(),
                setup_token.to_string(),
            )
            .await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < MAX_FAILS {
                sleep(Duration::from_secs(AYLA_CONNECTION_CHECK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(_) => {
                debug!("Alya accepted device 🎉");
                WifiPairing::log("Alya accepted device 🎉".to_string());
                self.check_device_connected().await
            }
            Err(err) => self.handle_error(err.to_string()),
        };
    }

    pub async fn check_device_connected(&'static mut self) {
        self.set_state(WifiPairingState::PollingDeviceOnAyla);
        let dsn = self.ayla_device_info().dsn().unwrap().to_owned();
        let access_token = self.access_token().unwrap().to_owned();
        let device_url = self.ayla_device_info().device_url().to_owned();
        let mut fail_count = 0;
        let mut result: Result<String, Box<dyn Error + Send + Sync>> = Err("".into());
        while fail_count < AYLA_CONNECTION_CHECK_MAX_FAILS && result.is_err() {
            debug!("checking device with DSN {} on Ayla", &dsn);
            WifiPairing::log(format!("checking device with DSN {} on Ayla", &dsn));
            result = get_device(
                device_url.to_string(),
                dsn.to_string(),
                access_token.to_string(),
            )
            .await;
            if result.is_err() {
                fail_count += 1;
            }
            if result.is_err() && fail_count < AYLA_CONNECTION_CHECK_MAX_FAILS {
                sleep(Duration::from_secs(AYLA_CONNECTION_CHECK_PAUSE_SECS)).await
            }
        }
        match result {
            Ok(dsn) => {
                debug!("Setting reporting periods");
                WifiPairing::log("Setting reporting periods".to_string());
                CloudCore::shared().set_reporting_periods(dsn.clone()).await;
                debug!("Setting time zone");
                WifiPairing::log("Setting time zone".to_string());
                _ = CloudCore::shared().set_device_time_zone(dsn.clone()).await;
                self.handle_connection_success(dsn);
            },
            Err(err) => self.handle_error(err.to_string()),
        }
    }

    pub fn handle_connection_success(&mut self, dsn: String) {
        self.set_state(WifiPairingState::Connected);
        debug!("🎉🎉🎉 DSN {} is connected to the internet!", &dsn);
        WifiPairing::log(format!("🎉🎉🎉 DSN {} is connected to the internet!", &dsn));
        if let Some(success_callback) = self.result_callback() {
            success_callback(Ok(dsn))
        }
        self.abort_runtime_task();
    }

    pub fn handle_error(&mut self, error: String) {
        // TODO: For now do not try to log the device info. It is causing crashes at times
        WifiPairing::log(format!("Device failed with error: {}", &error));
        error!("Device {:#?} failed with error: {}", self.ayla_device_info(), &error);
        self.error_tracker().set_error(error.clone());
        if let Some(err_callback) = self.result_callback() {
            err_callback(Err(error.into()))
        }
        self.abort_runtime_task();
    }
}

fn random_ayla_token() -> String {
    let base: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let length: usize = 8;
    let mut rng = rand::thread_rng();

    let token: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..base.len());
            base[idx] as char
        })
        .collect();
    token
}
