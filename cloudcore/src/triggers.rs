#[cfg(feature = "library")]
use std::collections::HashMap;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::str::FromStr;
#[cfg(feature = "library")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "library")]
use std::sync::atomic::AtomicUsize;
#[cfg(feature = "library")]
use std::sync::atomic::Ordering::Relaxed;
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use log::error;
#[cfg(feature = "library")]
use log::warn;
#[cfg(feature = "library")]
use mantle_utilities::RUNTIME;
#[cfg(feature = "library")]
use reqwest::Method;
#[cfg(feature = "library")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "library")]
use crate::cloudcore::CloudCore;
#[cfg(feature = "library")]
use crate::cloudcore_client::{CloudCoreBaseURL, CloudCoreRequest};
#[cfg(feature = "library")]
use crate::notifications::notifications::NotificationService;
#[cfg(feature = "library")]
use crate::properties::property::{PROPS_PATH_PARAMS_DSN, PROPS_PATH_PARAMS_PROP_NAME};
#[cfg(feature = "library")]
use crate::properties::trigger::{IoTTrigger, IoTTriggerApp};
#[cfg(feature = "library")]
use crate::properties::trigger::{TriggerAppRequest, TriggerRequest};
#[cfg(feature = "library")]
use crate::properties::trigger::{ERROR_NOTIFICATION_PROPERTY_NAME, ERROR_NOTIFICATION_PROPERTY_TYPE, ERROR_NOTIFICATION_TRIGGER_COMPARE_TYPE, ERROR_NOTIFICATION_TRIGGER_TYPE};
#[cfg(feature = "library")]
use crate::urls::{AYLA_PROPS_TRIGGERS_JSON, AYLA_TRIGGER_APP, AYLA_TRIGGER_APPS_JSON,
                  AYLA_TRIGGER_JSON, PROPS_PATH_PARAMS_TRIGGER_APP_KEY,
                  PROPS_PATH_PARAMS_TRIGGER_KEY};

#[cfg(feature = "library")]
impl CloudCore {
    pub async fn create_error_push_triggers_callback(
        &'static self,
        device_nicknames: HashMap<String, String>,
        device_id: String,
        application_id: String,
        channel_id: Option<String>,
        registration_id: Option<String>,
        service: String,
        errors: HashMap<String, HashMap<u32, String>>,
        callback: fn(result: Result<Vec<IoTTrigger>, Box<dyn Error>>),
    ) {
        debug!("DEBUG: create_error_push_triggers_callback");
        #[derive(Serialize, Debug)]
        pub struct AndroidDeviceRequest {
            android_device_id: String,
        }
        #[derive(Serialize, Debug)]
        pub struct IOSDeviceRequest {
            ios_device_id: String,
        }
        let fcns = Arc::new(Mutex::new(vec![]));
        let channel_id = channel_id.unwrap_or("".to_string());
        let service = NotificationService::from_str(service.as_str()).unwrap_or(NotificationService::PushiOS);
        let mut error = None;
        let device_json = if service == NotificationService::PushiOS {
            let device = IOSDeviceRequest {
                ios_device_id: device_id
            };
            match serde_json::to_string(&device) {
                Ok(str) => str,
                Err(err) => {
                    error = Some(err);
                    "".to_string()
                }
            }
        } else {
            let device = AndroidDeviceRequest {
                android_device_id: device_id
            };
            match serde_json::to_string(&device) {
                Ok(str) => str,
                Err(err) => {
                    error = Some(err);
                    "".to_string()
                }
            }
        };
        if let Some(e) = error {
            callback(Err(e.into()));
            return;
        }
        if (service == NotificationService::PushAndroidFcm || service == NotificationService::PushiOS) && registration_id.is_none() {
            callback(Err("Missing registration_id".into()));
        }
        let registration_id = registration_id.unwrap_or("".to_string());
        let len = Arc::new(Mutex::new(AtomicUsize::new(0)));
        let device_nicknames = Arc::new(device_nicknames);
        let errors = Arc::new(errors);
        for (dsn, nickname) in device_nicknames.iter() {
            let errors = Arc::clone(&errors);
            let dsn_errors = errors.get(dsn.clone().as_str()).unwrap().clone();
            dsn_errors.iter().for_each(|d_e| {
                let nickname = Arc::new(nickname.clone());
                let dsn = Arc::new(dsn.clone());
                if let Some(mut l) = len.lock().ok() {
                    let len = l.get_mut().clone() + 1;
                    l.store(len, Relaxed);
                }
                let code = Arc::new(d_e.0.clone());
                let message = Arc::new(d_e.1.to_string());
                debug!("Success transforming error message info {}", &message);
                let app_request = match service {
                    NotificationService::PushAndroidFcm => TriggerAppRequest::fcm_request(
                        registration_id.to_string(),
                        application_id.to_string(),
                        message.to_string(),
                        Some("normal".to_string()),
                        device_json.to_string(),
                    ),
                    NotificationService::PushAndroidBaidu => TriggerAppRequest::baidu_request(
                        application_id.to_string(),
                        channel_id.to_string(),
                        message.to_string(),
                        Some("normal".to_string()),
                        device_json.to_string(),
                    ),
                    NotificationService::PushiOS => TriggerAppRequest::ios_request(
                        registration_id.to_string(),
                        application_id.to_string(),
                        message.to_string(),
                        Some("normal".to_string()),
                        device_json.to_string(),
                    ),
                };
                let app_request = Arc::new(app_request);
                let fcns = Arc::clone(&fcns);
                let nickname = Arc::clone(&nickname);
                let code = Arc::clone(&code);
                let len = Arc::clone(&len);
                RUNTIME.spawn(async move {
                    let fcn = self.create_trigger_and_app(
                        dsn.to_string(),
                        ERROR_NOTIFICATION_PROPERTY_NAME.to_string(),
                        TriggerRequest::new(
                            nickname.to_string(),
                            ERROR_NOTIFICATION_PROPERTY_NAME.to_string(),
                            ERROR_NOTIFICATION_TRIGGER_COMPARE_TYPE.to_string(),
                            ERROR_NOTIFICATION_TRIGGER_TYPE.to_string(),
                            code.to_string(),
                            true,
                            ERROR_NOTIFICATION_PROPERTY_TYPE.to_string(),
                        ),
                        app_request.as_ref().clone(),
                    );
                    let r = fcn.await;
                    if let Some(mut fcns) = fcns.lock().ok() {
                        match r {
                            Ok(t) => {
                                fcns.push(t);
                            }
                            Err(e) => {
                                callback(Err(e));
                            }
                        }
                        if let Some(l) = len.lock().ok() {
                            if fcns.len() == l.load(Relaxed) {
                                callback(Ok(fcns.to_vec()));
                            }
                        }
                    }
                });
            })
        }
    }

    pub async fn create_trigger_and_app(
        &self,
        dsn: String,
        property_name: String,
        trigger_request: TriggerRequest,
        trigger_app_request: TriggerAppRequest,
    ) -> Result<IoTTrigger, Box<dyn Error>> {
        let mut trigger = self.create_trigger(
            dsn, property_name, trigger_request,
        ).await?;
        let trigger_app = self.create_trigger_app(trigger.key,
                                                  trigger_app_request).await?;
        trigger.add_trigger_app(trigger_app);
        Ok(trigger)
    }

    pub async fn create_trigger(
        &self,
        dsn: String,
        property_name: String,
        trigger_request: TriggerRequest,
    ) -> Result<IoTTrigger, Box<dyn Error>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerResponse {
            trigger: IoTTrigger,
        }
        #[derive(Serialize, Debug)]
        struct TriggerRequestWrapper {
            id: u32,
            trigger: TriggerRequest,
        }
        let request_body = TriggerRequestWrapper {
            id: 0,
            trigger: trigger_request,
        };
        let endpoint = String::from(AYLA_PROPS_TRIGGERS_JSON)
            .replace(PROPS_PATH_PARAMS_DSN, &dsn)
            .replace(PROPS_PATH_PARAMS_PROP_NAME, &property_name);

        let response = self.send_request(
            CloudCoreRequest {
                base_url: CloudCoreBaseURL::DEVICE,
                endpoint,
                method: Method::POST,
                requires_auth: true,
                body: Some(request_body),
            }
        ).await?;

        let triggers_payload = response.json::<TriggerResponse>().await?;
        Ok(triggers_payload.trigger)
    }

    pub async fn create_trigger_app(
        &self,
        trigger_key: u32,
        trigger_app_request: TriggerAppRequest,
    ) -> Result<IoTTriggerApp, Box<dyn Error>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerAppResponse {
            trigger_app: IoTTriggerApp,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerAppRequestWrapper {
            trigger_id: u32,
            trigger_app: TriggerAppRequest,
        }
        let request_body = TriggerAppRequestWrapper {
            trigger_id: trigger_key,
            trigger_app: trigger_app_request,
        };
        let endpoint = String::from(AYLA_TRIGGER_APPS_JSON).replace(
            PROPS_PATH_PARAMS_TRIGGER_KEY,
            &*trigger_key.to_string(),
        );

        let response = self.send_request(
            CloudCoreRequest {
                base_url: CloudCoreBaseURL::DEVICE,
                endpoint,
                method: Method::POST,
                requires_auth: true,
                body: Some(request_body),
            }).await?;

        let triggers_payload = response.json::<TriggerAppResponse>().await?;
        Ok(triggers_payload.trigger_app)
    }

    pub async fn delete_all_triggers(
        &mut self,
        dsn: String,
        prop_name: String,
    ) -> Result<(), Box<dyn Error>> {
        let triggers = self.fetch_triggers(dsn, prop_name).await?;

        for trigger in triggers {
            let _res = self.delete_trigger(trigger.key).await;
        }
        Ok(())
    }

    pub async fn delete_trigger(
        &mut self,
        trigger_key: u32,
    ) -> Result<(), Box<dyn Error>> {
        let endpoint = String::from(AYLA_TRIGGER_JSON)
            .replace(PROPS_PATH_PARAMS_TRIGGER_KEY, &*trigger_key.to_string());

        let response = self.send_request(
            CloudCoreRequest {
                base_url: CloudCoreBaseURL::DEVICE,
                endpoint,
                method: Method::DELETE,
                requires_auth: true,
                body: Some(""),
            }
        ).await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Unable to remove trigger".into())
        }
    }

    pub async fn delete_all_trigger_apps_by_device_id(
        &mut self,
        dsn: String,
        prop_name: String,
        device_id: String,
    ) -> Result<(), Box<dyn Error>> {
        let trigger_apps = self.fetch_trigger_apps_for_device_id(dsn, prop_name, device_id).await?;
        if trigger_apps.len() > 0 {
            let mut all_success = true;
            for trigger in trigger_apps {
                let res = self.delete_trigger_app(trigger.key).await;
                all_success = all_success && res.is_ok();
            }
            if !all_success {
                debug!("Some trigger apps may not have been deleted.")
            }
            Ok(())
        } else {
            debug!("No trigger apps to delete.");
            Err("No apps to delete".into())
        }
    }

    pub async fn delete_trigger_app(
        &self,
        trigger_app_key: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let endpoint = String::from(AYLA_TRIGGER_APP)
            .replace(PROPS_PATH_PARAMS_TRIGGER_APP_KEY, &*trigger_app_key.to_string());

        let response = self.send_request(
            CloudCoreRequest {
                base_url: CloudCoreBaseURL::DEVICE,
                endpoint,
                method: Method::DELETE,
                requires_auth: true,
                body: Some(""),
            }
        ).await?;
        Ok(response.status().is_success())
    }

    pub async fn fetch_triggers(
        &mut self,
        dsn: String,
        prop_name: String,
    ) -> Result<Vec<IoTTrigger>, Box<dyn Error>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerResponse {
            trigger: IoTTrigger,
        }

        let endpoint = String::from(AYLA_PROPS_TRIGGERS_JSON)
            .replace(PROPS_PATH_PARAMS_DSN, &dsn)
            .replace(PROPS_PATH_PARAMS_PROP_NAME, &prop_name);

        let body: Option<String> = None;
        let response = self.send_request(
            CloudCoreRequest {
                base_url: CloudCoreBaseURL::DEVICE,
                endpoint,
                method: Method::GET,
                requires_auth: true,
                body,
            }
        ).await?;

        let mut triggers = vec![];
        let triggers_payload = response.json::<Vec<TriggerResponse>>().await?;
        triggers_payload.into_iter().for_each(|trigger_response| {
            triggers.push(trigger_response.trigger)
        });
        Ok(triggers)
    }

    pub async fn update_all_trigger_apps_registration_id(
        &mut self,
        dsn: String,
        prop_name: String,
        device_id: String,
        registration_id: String,
    ) -> Result<(), Box<dyn Error>> {
        let triggers = self.fetch_trigger_apps_for_device_id(dsn, prop_name, device_id).await?;
        let trigger_count = triggers.len();
        if trigger_count > 0 {
            let mut all_success = true;
            for trigger in triggers {
                let res = self.update_trigger_app_registration_id(&trigger, registration_id.to_string()).await;
                all_success = all_success && match res {
                    Ok(_) => {
                        true
                    }
                    Err(e) => {
                        error!("Error updating registration_id: {:?}", e.to_string());
                        false
                    }
                }
            }
            if !all_success {
                warn!("Some trigger apps may not have updated.")
            }
            Ok(())
        } else {
            debug!("No trigger apps to update.");
            Err("No apps to update".into())
        }
    }

    pub async fn update_trigger_app(
        &self,
        trigger_app_key: u32,
        trigger_app_request: TriggerAppRequest,
    ) -> Result<IoTTriggerApp, Box<dyn Error>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerAppResponse {
            trigger_app: IoTTriggerApp,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct TriggerAppRequestWrapper {
            trigger_app: TriggerAppRequest,
        }
        let request_body = TriggerAppRequestWrapper {
            trigger_app: trigger_app_request.clone(),
        };
        let endpoint = String::from(AYLA_TRIGGER_APP).replace(
            PROPS_PATH_PARAMS_TRIGGER_APP_KEY,
            &*trigger_app_key.to_string(),
        );

        let req = CloudCoreRequest {
            base_url: CloudCoreBaseURL::DEVICE,
            endpoint: endpoint.to_string(),
            method: Method::PUT,
            requires_auth: true,
            body: Some(request_body),
        };
        let response = self.send_request(req).await?;
        let triggers_payload = response.json::<TriggerAppResponse>().await?;
        Ok(triggers_payload.trigger_app)
    }

    async fn update_trigger_app_registration_id(
        &self,
        trigger_app: &IoTTriggerApp,
        registration_id: String,
    ) -> Result<(), Box<dyn Error>> {
        let app = trigger_app.clone();
        let iot = IoTTriggerApp {
            param1: Some(registration_id),
            ..app
        };
        let update_result = self.update_trigger_app(
            trigger_app.key,
            TriggerAppRequest::from_trigger_app(iot),
        ).await;
        match update_result {
            Ok(_trigger_app) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn fetch_trigger_apps_for_device_id(
        &mut self,
        dsn: String,
        prop_name: String,
        device_id: String,
    ) -> Result<Vec<IoTTriggerApp>, Box<dyn Error>> {
        let triggers = self.fetch_triggers(
            dsn,
            prop_name,
        ).await?;
        let mut trigger_apps: Vec<IoTTriggerApp> = vec![];
        triggers.into_iter()
            .for_each(|trigger| {
                trigger_apps.append(&mut trigger.get_trigger_apps_for_device(device_id.to_string()))
            });
        Ok(
            trigger_apps
        )
    }

    // pub fn updates_trigger_apps_registration_id(
    //     trigger_apps: Vec<IoTTriggerApp>,
    //     registration_id: String,
    // ) -> Result<bool, Box<MantleError>> {
    //     let len = trigger_apps.len();
    //     let tas = Arc::new(trigger_apps);
    //     let reg = Arc::new(registration_id);
    //     let threads: Vec<_> = (0..len)
    //         .map(|thread_id| {
    //             let tas = Arc::clone(&tas);
    //             let reg = Arc::clone(&reg);
    //             let handle = Handle::current();
    //             spawn(move || {
    //                 let cc = CloudCore::shared();
    //                 handle.block_on(async move {
    //                     let ta = &tas[thread_id];
    //                     let result = cc.update_trigger_app_registration_id(
    //                         ta,
    //                         reg.to_string(),
    //                     ).await;
    //                     match result {
    //                         Ok(_) => { debug!("update_trigger_apps_registration_id(key: {}, reg: {}): SUCCESS", ta.key, reg.to_string()) }
    //                         Err(e) => { error!("Error updating registration id in trigger apps. {:?}", e) }
    //                     }
    //                 });
    //             })
    //         })
    //         .collect();
    //     let mut all_success = true;
    //     for t in threads {
    //         let success = t.join().is_ok();
    //         all_success = all_success && success;
    //     }
    //     Ok(all_success)
    // }
}

