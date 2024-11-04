use std::error::Error;
use std::time::Duration;
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::pairing::wifi_network::WifiNetwork;
use crate::{urls, WifiPairing};

static LAN_CONNECT_TIMEOUT: u64 = 10;
static INTERNET_CONNECT_TIMEOUT: u64 = 20;

pub async fn fetch_dsn(ip_address: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    #[derive(Deserialize, Debug)]
    struct AylaDeviceStatus {
        api_version: Option<String>,
        build: Option<String>,
        device_service: Option<String>,
        dsn: Option<String>,
        mtime: Option<u32>,
        version: Option<String>
    }
    let url = format!("http://{}/status.json", ip_address);
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(LAN_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .get(url.to_string())
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let status_payload = response.json::<AylaDeviceStatus>().await?;
        WifiPairing::log(format!("Response: {:#?}", &status_payload));
        Ok(status_payload.dsn.unwrap())
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}

pub async fn fetch_wifi_networks(ip_address: String) -> Result<Vec<WifiNetwork>, Box<dyn Error + Send + Sync>> {
    #[derive(Deserialize, Debug)]
    struct WifiScan {
        mtime: Option<u32>,
        results: Option<Vec<WifiNetwork>>
    }
    #[derive(Deserialize, Debug)]
    struct WifiResults {
        wifi_scan: Option<WifiScan>
    }
    let url = format!("http://{}/wifi_scan_results.json", ip_address);
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(LAN_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .get(url.to_string())
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let text = response.text().await?;
        let scan_payload: WifiResults = serde_json::from_str(&text)?;
        //WifiPairing::log(format!("Response: {:#?}", &scan_payload));
        Ok(scan_payload.wifi_scan.unwrap().results.unwrap())
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}

pub async fn start_wifi_scan(ip_address: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("http://{}/wifi_scan.json", ip_address);
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(LAN_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .post(url.to_string())
        .json("{}")
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        WifiPairing::log("Response was a success".to_string());
        Ok(())
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}

pub async fn send_wifi_credentials_to_device(ip_address: String, wifi_network: WifiNetwork, setup_token: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut query: Vec<(&str, String)> = vec![
        ("ssid", wifi_network.ssid().unwrap().to_string()),
        ("setup_token", setup_token),
        ("language", String::from("en")),
    ];
    if let Some(key) = wifi_network.password() {
        if !key.is_empty() {
            query.push(("key", key.to_string()));
        }
    }
    let url = format!("http://{}/wifi_connect.json", ip_address);
    WifiPairing::log(format!("Sending request to: {} with query: {:#?}", &url, &query));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(LAN_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .post(url.to_string())
        .query(&query)
        .json("{}")
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let text = response.text().await?;
        WifiPairing::log(format!("Response was a success: {}", &text));
        debug!("Success resp: {}", &text);
        Ok(())
    } else {
        let text = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &text));
        Err(text.into())
    }
}

pub async fn stop_device_access_point(ip_address: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("http://{}/wifi_stop_ap.json", ip_address);
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(LAN_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .put(url.to_string())
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if !response.status().is_success() {
        // Just log the error. Don't care if it actually failed
        let error = response.text().await?;
        error!("Error stopping access point: {}", error);
        WifiPairing::log(format!("Error stopping access point: {}", error));
    }
    Ok(())
}

// Does not work for China
/*pub async fn try_internet_request() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://captive.apple.com/");
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(INTERNET_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .get(url.to_string())
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        WifiPairing::log("Response was a success".to_string());
        Ok(())
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}*/

pub async fn ayla_device_handshake(ayla_device_url: String, dsn: String, access_token: String, setup_token: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[derive(Serialize, Debug)]
    struct RequestDevice {
        dsn: String,
        setup_token: String,
    }
    #[derive(Serialize, Debug)]
    struct HandshakeRequest {
        device: RequestDevice
    }
    #[derive(Deserialize, Debug)]
    struct ResponseDevice {
        dsn: String,
    }
    #[derive(Deserialize, Debug)]
    struct HandshakeResponse {
        device: ResponseDevice,
    }
    let handshake_request = HandshakeRequest {
        device: RequestDevice {
            dsn: dsn.to_owned(),
            setup_token,
        },
    };
    let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, access_token);
    let url = format!("{}{}", ayla_device_url, urls::AYLA_DEVICE_JSON);
    debug!("Handshaking to: {}", &url);
    WifiPairing::log(format!("Sending request to: {} for handshake with Ayla", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(INTERNET_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .post(url.to_string())
        .header(urls::AUTHORIZATION_HEADER, auth_bearer)
        .json(&handshake_request)
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let handshake_payload = response.json::<HandshakeResponse>().await?;
        if handshake_payload.device.dsn == dsn {
            WifiPairing::log("Response was a success. Returned correct DSN as well".to_string());
            Ok(())
        } else {
            WifiPairing::log(format!("Wrong DSN returned. Expected {}, got {}", &dsn, &handshake_payload.device.dsn));
            Err(format!("Wrong DSN returned. Expected {}, got {}", dsn, handshake_payload.device.dsn).into())
        }
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}

pub async fn get_device(ayla_device_url: String, dsn: String, access_token: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, access_token);
    let url = format!("{}/apiv1/dsns/{}.json", ayla_device_url, &dsn);
    debug!("getting device with url {}", &url);
    WifiPairing::log(format!("Sending request to: {} to get device", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(INTERNET_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .get(url.to_string())
        .header(urls::AUTHORIZATION_HEADER, auth_bearer)
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let text = response.text().await?;
        WifiPairing::log(format!("Device found on Ayla: {:#?}", &text));
        // Maybe deserialize to Device struct
        debug!("text from get_device: {}", text);
        Ok(dsn)
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}

// Not sure if this has any benefit over just trying to get the device. But keeping it here for now.
#[allow(dead_code)]
pub async fn device_connected(ayla_device_url: String, dsn: String, setup_token: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query: Vec<(&str, String)> = vec![("dsn", dsn), ("setup_token", setup_token)];
    #[derive(Deserialize, Debug)]
    struct ConnectedDevice {
        lan_ip: String,
        registration_type: String,
        connected_at: String,
        device_type: String,
    }
    #[derive(Deserialize, Debug)]
    struct ConnectedResponse {
        device: ConnectedDevice,
    }
    let url = format!("{}/apiv1/devices/connected.json", ayla_device_url);
    WifiPairing::log(format!("Sending request to: {}", &url));
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(INTERNET_CONNECT_TIMEOUT))
        .build()?;
    let response = client
        .get(url.to_string())
        .query(&query)
        .send()
        .await?;
    WifiPairing::log(format!("Received status code: {:#?}", response.status()));
    if response.status().is_success() {
        let connected_payload = response.json::<ConnectedResponse>().await?;
        WifiPairing::log(format!("Connected device payload: {:#?}", &connected_payload));
        debug!("Connected device payload: {:#?}", connected_payload);
        Ok(())
    } else {
        let error = response.text().await?;
        WifiPairing::log(format!("Request did not have success response: {}", &error));
        Err(error.into())
    }
}