mod devices;
mod time_zone;

pub use self::devices::IoTDevice;

#[cfg(feature = "library")]
use crate::ErrorUtil;
#[cfg(feature = "library")]
use crate::cloudcore::CloudCore;
#[cfg(feature = "library")]
use crate::urls;
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use serde::Deserialize;
#[cfg(feature = "library")]
use serde::Serialize;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::time::Duration;
#[cfg(feature = "library")]
use tokio::time::sleep;
#[cfg(feature = "library")]
use crate::cache::*;
#[cfg(feature = "library")]
use crate::devices::time_zone::TimeZone;
#[cfg(feature = "library")]
use crate::properties::value::IoTPropertyValue;

#[cfg(feature = "library")]
#[derive(Debug, Deserialize)]
struct IoTDeviceResponse {
    device: IoTDevice,
}

#[cfg(feature = "library")]
impl CloudCore {
    /// Fetch all the devices tied to the user's account.
    pub async fn fetch_all_devices(&mut self) -> Result<Vec<IoTDevice>, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let mut url = String::from(&self.session_params().device_url);
        url.push_str(urls::AYLA_DEVICE_JSON);

        let token = self.user_session.as_ref().unwrap().access_token();

        let auth_bearer = format!("auth_token {}", token);

        let client = self.client();

        let response = client
            .get(url)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        let text = response.text().await?;
        debug!("fetch devices payload: {:#?}", text);
        let devices_payload: Vec<IoTDeviceResponse> = serde_json::from_str(&text)?;

        let mut devices: Vec<IoTDevice> = vec![];
        devices_payload.into_iter().for_each(|obj| {
            debug!("{:?}", &obj.device);
            let dsn_result = obj.device.dsn();
            match dsn_result {
                None => {
                    debug!("cache creation skipped due to empty DSN");
                }
                Some(dsn) => {
                    if !self.cache.child_paths().contains_key(dsn) {
                        self.cache.make_dir_for_child(dsn).unwrap();
                        debug!("cache made for device");
                    }
                    devices.push(obj.device);
                }
            }
        });
        Ok(devices)
    }

    /// Fetch single device by using a Device Serial Number.
    pub async fn fetch_device_with_dsn(&mut self, dsn: String) -> Result<IoTDevice, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/dsns/{}.json", &self.session_params().device_url, &dsn);
        let response = self.client()
            .get(url.to_string())
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            let device_payload = response.json::<IoTDeviceResponse>().await?;
            let device = device_payload.device;
            debug!("{:?}", device);
            let dsn_result = device.dsn();
            match dsn_result {
                None => {
                    debug!("cache creation skipped due to empty DSN");
                }
                Some(dsn) => {
                    if !self.cache.child_paths().contains_key(dsn) {
                        self.cache.make_dir_for_child(dsn).unwrap();
                    }
                }
            }
            Ok(device.clone())
        } else {
            let error = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error)))
        }
    }

    pub async fn rename_device_with_dsn(&self, dsn: String, new_name: String) -> Result<(),  Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        #[derive(Debug, Serialize)]
        struct IoTDeviceRenameDevice {
            product_name: String,
        }
        #[derive(Debug, Serialize)]
        struct IoTDeviceRenameRequest {
            device: IoTDeviceRenameDevice,
        }
        let body = IoTDeviceRenameRequest {
            device: IoTDeviceRenameDevice {
                product_name: new_name
            }
        };
        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/dsns/{}.json", &self.session_params().device_url, &dsn);
        let response = self.client()
            .put(url.to_string())
            .json(&body)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send();

        let r = response.await;
        match r {
            Ok(_res) => {
                if _res.status().is_success() {
                    Ok(())
                } else {
                    Err(Box::new(ErrorUtil::generic_error()))
                }
            },
            Err(err) => {
                Err(Box::new(ErrorUtil::server_error(err.to_string())))
            }
        }
    }

    pub async fn reset_device(&self, key: u32) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/devices/{}/cmds/factory_reset.json", &self.session_params().device_url, key);

        let response = self.client()
            .put(url.to_string())
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let error = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error)))
        }
    }

    pub async fn unregister_device(&mut self, key: u32, dsn: String) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/devices/{}.json", &self.session_params().device_url, key);

        let response = self.client()
            .delete(url.to_string())
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            self.clear_device_cache(dsn.clone())
        } else {
            let error = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error)))
        }
    }

    pub async fn device_timezone_offset(&self, key: u32) -> Result<String, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/devices/{}/time_zones.json", &self.session_params().device_url, key);

        let response = self.client()
            .get(url.to_string())
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct TimeZoneResponse {
            time_zone: TimeZone
        }

        if response.status().is_success() {
            let tz_payload = response.json::<TimeZoneResponse>().await?;
            if let Some(offset) = tz_payload.time_zone.utc_offset {
                Ok(offset)
            } else {
                Err("no time zone offset".into())
            }
        } else {
            let error = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error)))
        }
    }

    pub async fn set_device_time_zone(&self, dsn: String) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", urls::AUTHORIZATION_BEARER, token);
        let url = format!("{}/apiv1/dsns/{}/time_zones.json", &self.session_params().device_url, dsn);

        #[derive(Serialize)]
        struct TimeZoneRequest {
            tz_id: String
        }

        let time_zone = self.cache.get_value("app".to_string(), "tz_id".to_string())?;
        let tz_id = match time_zone {
            CacheDataValue::StringValue(str) => str,
            _ => return Err(Box::new(ErrorUtil::invalid_format()))
        };

        let json = TimeZoneRequest {
            tz_id: tz_id.clone()
        };

        let response = self.client()
            .put(url.to_string())
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .json(&json)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct TimeZoneResponse {
            time_zone: TimeZone
        }

        if response.status().is_success() {
            let tz_payload = response.json::<TimeZoneResponse>().await?;
            if let Some(ayla_tz_id) = tz_payload.time_zone.tz_id {
                if ayla_tz_id == tz_id {
                    Ok(())
                } else {
                    Err("Ayla time zone does not match time zone sent".into())
                }
            } else {
                Err("no time zone offset".into())
            }
        } else {
            let error = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error)))
        }

    }

    pub async fn delete_device(&mut self, key: u32, dsn: String) -> Result<(), Box<dyn Error>> {
        self.reset_wifi_datapoint(dsn.clone()).await;
        let _ = self.rename_device_with_dsn(dsn.clone(), "Robot 1".to_string()).await;
        // TODO: For now just do these synchronously
        let _ = self.set_property_value(dsn.clone(), "GET_Ack_Response".to_string(), IoTPropertyValue::Str("".to_string()), "".to_string()).await;
        let _ = self.set_property_value(dsn.clone(), "SET_Quiet_Time".to_string(), IoTPropertyValue::Str("".to_string()), "".to_string()).await;
        let _ = self.set_property_value(dsn.clone(), "SET_Find_Device".to_string(), IoTPropertyValue::Int(0), "".to_string()).await;
        let _set_reporting_periods = self.set_reporting_periods(dsn.clone()).await;
        let _ = self.clear_schedules(key.clone()).await;
        let _ = self.delete_all_triggers(dsn.clone(), "Get_ERROR_Code".to_string()).await;
        self.unregister_device(key.clone(), dsn.clone()).await
    }

    pub async fn set_reporting_periods(&self, dsn: String) {
        if let Some(session) = self.user_session.as_ref() {
            let mut period_dock = 240;
            let mut period_undock = 20;
            if session.use_dev() {
                period_dock = 30;
                period_undock = 5;
            }
            // TODO: For now just do these synchronously
            let _ = self.set_property_value(dsn.clone(), "ReportPeriodDock".to_string(), IoTPropertyValue::Int(period_dock), "".to_string()).await;
            let _ = self.set_property_value(dsn.clone(), "ReportPeriodUndocked".to_string(), IoTPropertyValue::Int(period_undock), "".to_string()).await;
        }
    }

    pub async fn reset_wifi_datapoint(&self, dsn: String) {
        let _ = self.set_property_value(dsn.clone(), "SET_Reset_WiFi".to_string(), IoTPropertyValue::Int(1), "".to_string()).await.0;
        debug!("Sleeping for 5 seconds to allow Ayla to process reset wifi");
        sleep(Duration::from_secs(5)).await;
        let _ = self.set_property_value(dsn.clone(), "SET_Reset_WiFi".to_string(), IoTPropertyValue::Int(0), "".to_string()).await.0;
    }

    pub async fn factory_reset_device(&mut self, key: u32, dsn: String) -> Result<(), Box<dyn Error>> {
        self.set_property_value(dsn.clone(), "SET_Reset_Factory_Defaults".to_string(), IoTPropertyValue::Int(1), "".to_string()).await.0?;
        debug!("Sleeping for 60 seconds to allow Ayla to process everything");
        sleep(Duration::from_secs(60)).await;
        //let _ = self.delete_device_map(dsn.clone(), false, false).await;
        let _ = self.delete_device(key.clone(), dsn.clone()).await;
        Ok(())
    }

    pub async fn delete_device_map(&mut self, dsn: String, re_explore: bool, partial_delete: bool) -> Result<(), Box<dyn Error>> {
        let has_delete_map_prop = match self.get_properties(dsn.clone(), vec!["SET_DeleteMaps".to_string()], "".to_string()).await.0 {
            Ok(props) => !props.is_empty(),
            Err(_) => false
        };
        if has_delete_map_prop {
            let mut val = -1;
            if re_explore {
                val = if partial_delete { 1 } else { val };
            }
            let _ = self.set_property_value(dsn.clone(), "SET_DeleteMaps".to_string(), IoTPropertyValue::Int(val), "".to_string()).await.0;
        } else {
            self.set_property_value(dsn.clone(), "SET_Reset_Factory_Defaults".to_string(), IoTPropertyValue::Int(1), "".to_string()).await.0?;
        }
        debug!("Sleeping for 60 seconds to allow Ayla to process the set");
        sleep(Duration::from_secs(60)).await;
        // TODO: Maybe need to set "MapHasBeenReset" in MARD to 'true' as well
        Ok(())
    }

    pub fn clear_device_cache(&mut self, dsn: String) -> Result<(), Box<dyn Error>>  {
        if let Some(child_path) = self.cache.child_paths().clone().get(&dsn).clone() {
            if let Some(path) = child_path.to_str() {
                self.cache.remove_dir_for_child(&path.to_string())?;
                self.cache.remove_child_path(dsn.clone());
                Ok(())
            } else {
                Err(Box::new(ErrorUtil::server_error("Could not get a path from cache child path".to_string())))
            }
        } else {
            Ok(())
        }
    }
}