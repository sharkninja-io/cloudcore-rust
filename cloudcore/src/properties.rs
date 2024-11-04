pub mod datapoint;
pub mod property;
pub mod trigger;
pub mod value;

#[cfg(feature = "library")]
use crate::cloudcore::CloudCore;
#[cfg(feature = "library")]
use crate::io::{download_resource, write_to_disk};
#[cfg(feature = "library")]
use crate::properties::datapoint::{DataPointResponse, IoTDatapointMessage};
#[cfg(feature = "library")]
use crate::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMetadata};
#[cfg(feature = "library")]
use crate::properties::property::{
    IoTProperty, PROPS_PATH_PARAMS_DATAPOINT_ID, PROPS_PATH_PARAMS_DSN, PROPS_PATH_PARAMS_PROP_NAME,
};
#[cfg(feature = "library")]
use crate::properties::value::IoTPropertyValue;
#[cfg(feature = "library")]
use crate::urls::{
    AUTHORIZATION_BEARER, AUTHORIZATION_HEADER, AYLA_DATAPOINTS_FILTER_END_DATE_KEY,
    AYLA_DATAPOINTS_FILTER_SINCE_DATE_KEY, AYLA_DATAPOINTS_LIMIT_KEY, AYLA_PROPS_DATAPOINTS_JSON,
    AYLA_PROPS_JSON, AYLA_PROPS_MSG_DATAPOINTS_JSON, AYLA_PROPS_QUERY_PARAMS_KEY,
    AYLA_PROP_DATAPOINT_ID_JSON,
};
#[cfg(feature = "library")]
use chrono::DateTime;
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use reqwest::StatusCode;
#[cfg(feature = "library")]
use reqwest::{Body, Url};
#[cfg(feature = "library")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::fs;
#[cfg(feature = "library")]
use std::future::Future;
#[cfg(feature = "library")]
use std::path::Path;
#[cfg(feature = "library")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "library")]
use log::error;
#[cfg(feature = "library")]
use mantle_utilities::{ErrorType, MantleError, RUNTIME};
#[cfg(feature = "library")]
use tokio::fs::File;
#[cfg(feature = "library")]
use tokio_util::codec::{BytesCodec, FramedRead};
#[cfg(feature = "library")]
use crate::ErrorUtil;

#[cfg(feature = "library")]
static MAX_DATAPOINT_COUNT: u32 = 100;

#[cfg(feature = "library")]
impl CloudCore {
    pub async fn get_property(
        &self,
        dsn: String,
        prop: String,
        callback_id: String,
    ) -> (Result<Vec<IoTProperty>, Box<dyn Error>>, String) {
        self.get_properties(dsn, vec![prop], callback_id).await
    }

    pub async fn get_properties(
        &self,
        dsn: String,
        props: Vec<String>,
        callback_id: String,
    ) -> (Result<Vec<IoTProperty>, Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                let error = Box::new(ErrorUtil::user_session_not_found_error());
                return Err(error.into());
            }
            #[derive(Debug, Deserialize)]
            struct IoTPropertyListResponse {
                pub property: IoTProperty,
            }

            let mut url = String::from(&self.session_params().device_url);
            let endpoint = String::from(AYLA_PROPS_JSON).replace(PROPS_PATH_PARAMS_DSN, &dsn);
            url.push_str(&endpoint);

            let token = self.user_session.as_ref().unwrap().access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            let client = self.client();

            let mut query: Vec<(&str, String)> = vec![];
            props.into_iter().for_each(|prop| {
                let params = (AYLA_PROPS_QUERY_PARAMS_KEY, prop);
                query.push(params);
            });

            let response = client
                .get(url)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .query(&query)
                .send()
                .await?;

            if !response.status().is_success() {
                let error_payload = response.text().await?;
                let error = Box::new(ErrorUtil::server_error(error_payload));
                return Err(error.into());
            }
            let text = response.text().await?;
            debug!("properties: {}", &text);
            let properties_payload: Vec<IoTPropertyListResponse> =
                serde_json::from_str(text.as_ref())?; //response.json::<Vec<IoTPropertyListResponse>>().await?;

            let mut properties: Vec<IoTProperty> = vec![];
            properties_payload.into_iter().for_each(|obj| {
                debug!("{:?}", &obj.property);
                properties.push(obj.property);
            });

            Ok(properties)
        };
        let result = future.await;
        (result, callback_id)
    }

    /// 'to' Is exclusive
    pub async fn get_datapoints(
        &self,
        dsn: String,
        prop_name: String,
        count: Option<u32>,
        // It is up to the caller to make sure these dates are formatted correctly
        from: Option<String>,
        to: Option<String>,
        callback_id: String,
    ) -> (Result<Vec<IoTDatapoint>, Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                return Err(Box::new(ErrorUtil::user_session_not_found_error()).into());
            }
            let mut url = String::from(&self.session_params().device_url);
            let endpoint = String::from(AYLA_PROPS_DATAPOINTS_JSON)
                .replace(PROPS_PATH_PARAMS_DSN, &dsn)
                .replace(PROPS_PATH_PARAMS_PROP_NAME, &prop_name);
            url.push_str(&endpoint);

            let mut limit = MAX_DATAPOINT_COUNT;
            if let Some(count) = count {
                if count > 0 && count < MAX_DATAPOINT_COUNT {
                    limit = count
                }
            }
            let limit_query = vec![(AYLA_DATAPOINTS_LIMIT_KEY, limit)];

            let mut date_query = vec![];
            if let Some(from) = from {
                date_query.push((AYLA_DATAPOINTS_FILTER_SINCE_DATE_KEY, from));
            }
            if let Some(to) = to {
                date_query.push((AYLA_DATAPOINTS_FILTER_END_DATE_KEY, to));
            }

            let session = self.user_session.as_ref().unwrap();
            let token = session.access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            #[derive(Debug, Deserialize, Serialize)]
            struct DataPointResponse {
                datapoint: IoTDatapoint,
            }

            // Use if pagination is ever needed:
            // https://docs.aylanetworks.com/reference/get-datapoints-by-dsn
            #[derive(Debug, Deserialize)]
            struct MetaResponse {
                previous_page: Option<String>,
                next_page: Option<String>,
                current_page_number: Option<u32>,
            }

            let client = self.client();

            let response = client
                .get(url)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .query(&limit_query)
                .query(&date_query)
                .send()
                .await?;

            if !response.status().is_success() {
                let error_payload = response.text().await?;
                return Err(Box::new(ErrorUtil::server_error(error_payload)).into());
            }

            let mut datapoints = vec![];
            let datapoints_payload = response.json::<Vec<DataPointResponse>>().await?;
            datapoints_payload
                .into_iter()
                .for_each(|datapoint_response| {
                    let datapoint = datapoint_response.datapoint;
                    datapoints.push(datapoint)
                });
            Ok(datapoints)
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn get_file_property(
        &self,
        dsn: String,
        prop_name: String,
        callback_id: String,
    ) -> (Result<IoTDatapointFile, Box<dyn Error>>, String) {
        let cb_id = callback_id.clone();
        let future = async move {
            let mut datapoints = self
                .get_datapoints(
                    dsn.to_string(),
                    prop_name.to_string(),
                    None,
                    None,
                    None,
                    cb_id.to_string(),
                )
                .await.0?;
            let string_to_int = |date: Option<&String>| {
                if let Some(date) = date {
                    if let Some(date) = DateTime::parse_from_rfc3339(date).ok() {
                        return date.timestamp() as u64;
                    }
                }
                0 as u64
            };
            datapoints.sort_by(|a, b| {
                let a_timestamp = string_to_int(a.created_at());
                let b_timestamp = string_to_int(b.created_at());
                b_timestamp.cmp(&a_timestamp)
            });
            if datapoints.is_empty() {
                return Err(Box::new(ErrorUtil::datapoints_missing()).into());
            }
            let property = datapoints.get(0).unwrap();
            let val = property.value();
            let url = val.string_value();
            if url.is_none() {
                return Err(Box::new(ErrorUtil::property_not_string_type()).into());
            }
            let url = url.unwrap();
            self.get_datapoint_with_file_url(
                url.to_string(),
                dsn.to_owned(),
                prop_name.to_owned(),
                cb_id.to_owned(),
            )
            .await.0
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn get_file_properties_callback(
        &'static self,
        dsn: String,
        prop_names: Vec<String>,
        callback_id: String,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<dyn Error>>, String))
    ) {
        let fv = Arc::new(Mutex::new(vec![]));
        let prop_names = Arc::new(prop_names);
        let callback_id = Arc::new(callback_id);
        let dsn = Arc::new(dsn);
        let len = prop_names.len();
        for i in 0..len {
            let prop_names = Arc::clone(&prop_names);
            let prop_name = prop_names[i].to_string();
            let callback_id = Arc::clone(&callback_id);
            let dsn = Arc::clone(&dsn);
            let fv = Arc::clone(&fv);
            RUNTIME.spawn(async move {
                let f =
                    self.get_file_property(dsn.to_string(), prop_name.to_string(), "".to_string());
                let r = run_file_future_file(f).await;
                if let Some(mut fv) = fv.lock().ok() {
                    fv.push(r);
                    if fv.len() == len {
                        callback((Ok(fv.to_vec()), callback_id.to_string()));
                    }
                }
            });
        }
    }

    pub async fn get_datapoint_with_file_url(
        &self,
        url: String,
        dsn: String,
        prop_name: String,
        callback_id: String,
    ) -> (Result<IoTDatapointFile, Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                return Err(Box::new(ErrorUtil::user_session_not_found_error()).into());
            }
            let token = self.user_session.as_ref().unwrap().access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            #[derive(Debug, Deserialize)]
            struct IoTDatapointFileResponse {
                pub datapoint: IoTDatapointFile,
            }

            let client = self.client();

            let response = client
                .get(url)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .send()
                .await?;

            if !response.status().is_success() {
                let error_payload = response.text().await?;
                return Err(Box::new(ErrorUtil::server_error(error_payload)).into());
            }

            let datapoint_payload = response.json::<IoTDatapointFileResponse>().await?;
            let mut datapoint = datapoint_payload.datapoint;

            if let Some(cache_dir) = self.cache.parent_path().to_str() {
                if let Some(file_name) = Path::new(datapoint.value()).file_name() {
                    if let Some(file_name) = file_name.to_str() {
                        let file_path = format!(
                            "{}/{}/properties/{}/datapoints/{}",
                            cache_dir, dsn, prop_name, file_name
                        );
                        download_resource(datapoint.file(), &file_path).await?;
                        datapoint.set_local_file(file_path);
                        Ok(datapoint)
                    } else {
                        Err(Box::new(ErrorUtil::local_file_name_error()).into())
                    }
                } else {
                    Err(Box::new(ErrorUtil::local_file_name_error()).into())
                }
            } else {
                Err(Box::new(ErrorUtil::cached_directory_error()).into())
            }
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn get_message_property(
        &self,
        dsn: String,
        prop_name: String,
        callback_id: String,
    ) -> (Result<IoTDatapointMessage, Box<dyn Error>>, String) {
        let cb_id = callback_id.clone();
        let future = async move {
            let properties = self
                .get_properties(
                    dsn.to_owned(),
                    vec![prop_name.to_owned()],
                    cb_id.to_owned(),
                )
                .await.0?;
            if properties.is_empty() {
                return Err(Box::new(ErrorUtil::property_not_found()).into());
            }
            let property = properties.get(0).unwrap();
            let val = property.value();
            if val.is_none() {
                return Err(Box::new(ErrorUtil::url_not_found()).into());
            }
            if property.base_type() != "message" {
                return Err(Box::new(ErrorUtil::property_not_message_type()).into());
            }
            let url = val.unwrap().string_value();
            if url.is_none() {
                return Err(Box::new(ErrorUtil::property_not_string_type()).into());
            }
            let components: Vec<&str> = url.unwrap().split("/").collect();
            if components.len() == 1
                || components.first().is_none()
                || components.first().unwrap() == url.unwrap()
            {
                return Err(Box::new(ErrorUtil::invalid_format()).into());
            }
            let datapoint_id = components.last().unwrap();
            self.get_datapoint_with_id(
                dsn.to_string(),
                datapoint_id.to_string(),
                prop_name.to_string(),
                cb_id.to_string(),
            )
            .await.0
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn get_message_properties_callback(
        &'static self,
        dsn: String,
        prop_names: Vec<String>,
        callback_id: String,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<dyn Error>>, String))
    ) {
        let fv = Arc::new(Mutex::new(vec![]));
        let prop_names = Arc::new(prop_names);
        let callback_id = Arc::new(callback_id);
        let dsn = Arc::new(dsn);
        let len = prop_names.len();
        for i in 0..len {
            let prop_names = Arc::clone(&prop_names);
            let prop_name = prop_names[i].to_string();
            let callback_id = Arc::clone(&callback_id);
            let dsn = Arc::clone(&dsn);
            let fv = Arc::clone(&fv);
            RUNTIME.spawn(async move {
                let f =
                    self.get_message_property(dsn.to_string(), prop_name.to_string(), "".to_string());
                let r = run_msg_future_msg(f).await;
                if let Some(mut fv) = fv.lock().ok() {
                    fv.push(r);
                    if fv.len() == len {
                        callback((Ok(fv.to_vec()), callback_id.to_string()));
                    }
                }
            });
        }
    }


    pub async fn get_datapoint_with_id(
        &self,
        dsn: String,
        datapoint_id: String,
        prop_name: String,
        callback_id: String,
    ) -> (Result<IoTDatapointMessage, Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                return Err(Box::new(ErrorUtil::user_session_not_found_error()).into());
            }
            let token = self.user_session.as_ref().unwrap().access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            let mut url = String::from(&self.session_params().device_url);
            let endpoint = String::from(AYLA_PROP_DATAPOINT_ID_JSON)
                .replace(PROPS_PATH_PARAMS_DSN, &dsn)
                .replace(PROPS_PATH_PARAMS_PROP_NAME, &prop_name)
                .replace(PROPS_PATH_PARAMS_DATAPOINT_ID, &datapoint_id);
            url.push_str(&endpoint);

            let client = self.client();

            let response = client
                .get(url)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .send()
                .await?;

            if !response.status().is_success() {
                let error_payload = response.text().await?;
                return Err(Box::new(ErrorUtil::server_error(error_payload)).into());
            }

            let datapoint_payload = response.json::<DataPointResponse>().await?;
            let datapoint = datapoint_payload.datapoint;

            if let Some(cache_dir) = self.cache.parent_path().to_str() {
                if let Some(file_name) = Path::new(&datapoint_id).file_name() {
                    if let Some(file_name) = file_name.to_str() {
                        let file_path = format!(
                            "{}/{}/properties/{}/datapoints/{}.message",
                            cache_dir, dsn, prop_name, file_name
                        );
                        let content = datapoint.value().string_value().unwrap();
                        match write_to_disk(
                            Path::new(&file_path),
                            bytes::Bytes::copy_from_slice(content.as_bytes()),
                        ) {
                            Ok(_) => {}
                            Err(err) => debug!("Error saving message content: {}", err),
                        };
                        let msg_dp = IoTDatapointMessage::new(datapoint, file_path);
                        Ok(msg_dp)
                    } else {
                        return Err(Box::new(ErrorUtil::local_file_name_error()).into());
                    }
                } else {
                    return Err(Box::new(ErrorUtil::local_file_name_error()).into());
                }
            } else {
                return Err(Box::new(ErrorUtil::cached_directory_error()).into());
            }
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn set_property_value(
        &self,
        dsn: String,
        prop_name: String,
        value: IoTPropertyValue,
        callback_id: String,
    ) -> (Result<(), Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                return Err(Box::new(ErrorUtil::user_session_not_found_error()).into());
            }
            let mut url = String::from(&self.session_params().device_url);
            let endpoint = String::from(AYLA_PROPS_DATAPOINTS_JSON)
                .replace(PROPS_PATH_PARAMS_DSN, &dsn)
                .replace(PROPS_PATH_PARAMS_PROP_NAME, &prop_name);
            url.push_str(&endpoint);

            let session = self.user_session.as_ref().unwrap();
            let token = session.access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            let client = self.client();

            let uuid = session.user_uuid().unwrap().to_owned();
            let datapoint = IoTDatapoint::new(
                value,
                IoTDatapointMetadata::new(uuid),
                None,
                None,
                None,
            );
            #[derive(Serialize)]
            struct CreateDataPointRequest {
                datapoint: IoTDatapoint,
            }
            let request_data = CreateDataPointRequest { datapoint };

            let response = client
                .post(url)
                .json(&request_data)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .send()
                .await?;

            if response.status() == StatusCode::CREATED {
                Ok(())
            } else {
                return Err(Box::new(ErrorUtil::create_datapoint_error(response.text().await?)).into());
            }
        };
        let result = future.await;
        (result, callback_id)
    }

    pub async fn get_file_property_as_files_callback(
        &'static self,
        dsn: String,
        prop_name: String,
        count: Option<u32>,
        from: Option<String>,
        to: Option<String>,
        callback_id: String,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<dyn Error>>, String))
    ) {
        let datapoints = self
            .get_datapoints(
                dsn.to_string(),
                prop_name.to_string(),
                count,
                from,
                to,
                "".to_string(),
            )
            .await.0;
        if let Some(err) = datapoints.as_ref().err() {
            callback((Err(Box::new(MantleError { error_type: ErrorType::MessageDatapointError, description: err.to_string() })), callback_id.clone()));
            return;
        }
        let datapoints = datapoints.unwrap();
        let len = datapoints.len();
        let dps = Arc::new(datapoints);
        let fv = Arc::new(Mutex::new(vec![]));
        let callback_id = Arc::new(callback_id);
        let prop_name = Arc::new(prop_name);
        let dsn = Arc::new(dsn);
        for i in 0..len {
            let dps = Arc::clone(&dps);
            let datapoint = &dps[i];
            let val = datapoint.value();
            let url = val.string_value().unwrap().to_string();
            let callback_id = Arc::clone(&callback_id);
            let prop_name = Arc::clone(&prop_name);
            let dsn = Arc::clone(&dsn);
            let fv = Arc::clone(&fv);
            RUNTIME.spawn(async move {
                let f = self.get_datapoint_with_file_url(
                    url,
                    dsn.to_string(),
                    prop_name.to_string(),
                    "".to_string(),
                );
                let r = run_file_future_file(f).await;
                if let Some(mut fv) = fv.lock().ok() {
                    fv.push(r);
                    if fv.len() == len {
                        callback((Ok(fv.to_vec()), callback_id.to_string()));
                    }
                }
            });
        };
    }

    pub async fn get_message_property_as_files_callback(
        &'static self,
        dsn: String,
        prop_name: String,
        count: Option<u32>,
        // It is up to the caller to make sure these dates are formatted correctly
        from: Option<String>,
        to: Option<String>,
        callback_id: String,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<dyn Error>>, String))
    ) {
        let datapoints = self
            .get_datapoints(
                dsn.to_string(),
                prop_name.to_string(),
                count,
                from,
                to,
                "".to_string(),
            )
            .await.0;
        if let Some(err) = datapoints.as_ref().err() {
            callback((Err(Box::new(MantleError { error_type: ErrorType::MessageDatapointError, description: err.to_string() })), callback_id.clone()));
            return;
        }
        let datapoints = datapoints.unwrap();
        let len = datapoints.len();
        let dps = Arc::new(datapoints);
        let fv = Arc::new(Mutex::new(vec![]));
        let callback_id = Arc::new(callback_id);
        let prop_name = Arc::new(prop_name);
        let dsn = Arc::new(dsn);
        for i in 0..len {
            let mut ok = true;
            let dps = Arc::clone(&dps);
            let datapoint = &dps[i];
            let val = datapoint.value();
            let url = val.string_value();
            if url.is_none() {
                debug!("Datapoint value URL is none, skipping");
                ok = false
            }
            let components: Vec<&str> = url.unwrap().split("/").collect();
            if components.len() == 1
                || components.first().is_none()
                || components.first().unwrap() == url.unwrap()
            {
                ok = false;
                debug!("Datapoint value URL components are not valid, skipping");
            }
            if ok {
                let datapoint_id = components.last().unwrap().to_string();
                let callback_id = Arc::clone(&callback_id);
                let prop_name = Arc::clone(&prop_name);
                let dsn = Arc::clone(&dsn);
                let fv = Arc::clone(&fv);
                RUNTIME.spawn(async move {
                    let f = self.get_datapoint_with_id(
                        dsn.to_string(),
                        datapoint_id,
                        prop_name.to_string(),
                        "".to_string().clone(),
                    );
                    let r = run_msg_future_msg(f).await;
                    if let Some(mut fv) = fv.lock().ok() {
                        fv.push(r);
                        if fv.len() == len {
                            callback((Ok(fv.to_vec()), callback_id.to_string()));
                        }
                    }
                });
            }
        }
    }

    pub async fn save_file(
        &self,
        dsn: String,
        prop_name: String,
        file_path: String,
        is_message: bool,
        callback_id: String,
    ) -> (Result<(), Box<dyn Error>>, String) {
        let future = async move {
            if self.user_session.is_none() {
                return Err(Box::new(ErrorUtil::user_session_not_found_error()).into());
            }
            let mut url = String::from(&self.session_params().device_url);
            let endpoint = if is_message {
                String::from(AYLA_PROPS_MSG_DATAPOINTS_JSON)
            } else {
                String::from(AYLA_PROPS_DATAPOINTS_JSON)
            }
            .replace(PROPS_PATH_PARAMS_DSN, &dsn)
            .replace(PROPS_PATH_PARAMS_PROP_NAME, &prop_name);
            url.push_str(&endpoint);

            let session = self.user_session.as_ref().unwrap();
            let token = session.access_token();
            let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

            let client = self.client();

            let response = client
                .post(url)
                .header(AUTHORIZATION_HEADER, auth_bearer.clone())
                .send()
                .await?;

            if response.status() != StatusCode::CREATED {
                return Err(Box::new(ErrorUtil::save_datapoint_error(response.text().await?)).into());
            }
            #[derive(Debug, Deserialize)]
            struct IoTDatapointFileResponse {
                pub datapoint: IoTDatapointFile,
            }
            let datapoint_payload = response.json::<IoTDatapointFileResponse>().await?;
            let datapoint = datapoint_payload.datapoint;
            let file = File::open(file_path.clone()).await?;
            let size = fs::metadata(Path::new(&file_path))?.len();
            let response = client
                .put(datapoint.file())
                .header("Content-Type", "application/octet-stream")
                .header("Content-Length", size)
                .body(file_to_body(file))
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(Box::new(ErrorUtil::send_file_stream(response.text().await?)).into());
            }
            let url = Url::parse(datapoint.value())?;
            let mut components = url.path_segments().unwrap().collect::<Vec<_>>();
            if components.len() > 2 {
                components.drain(0..1);
            }
            let location = components.join("/");
            let mut put_url = String::from(&self.session_params().device_url);
            put_url.push_str("/apiv1/");
            put_url.push_str(&location);
            let response = client
                .put(put_url)
                .header(AUTHORIZATION_HEADER, auth_bearer)
                .send()
                .await?;
            if !response.status().is_success() {
                let error = response.text().await?;
                return Err(Box::new(ErrorUtil::file_mark_complete_error(error.to_string())).into());
            }
            return Ok(());
        };
        let result = future.await;
        (result, callback_id)
    }
}

#[cfg(feature = "library")]
fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

#[cfg(feature = "library")]
async fn run_msg_future_msg(f: impl Future<Output = (Result<IoTDatapointMessage, Box<dyn Error>>, String)>) -> IoTDatapointMessage {
    match f.await.0 {
        Ok(m) => m,
        Err(e) => {
            error!("Error getting message datapoint: {}", e.to_string());
            IoTDatapointMessage::empty()
        }
    }
}

#[cfg(feature = "library")]
async fn run_file_future_file(f: impl Future<Output = (Result<IoTDatapointFile, Box<dyn Error>>, String)>) -> IoTDatapointFile {
    match f.await.0 {
        Ok(f) => f,
        Err(e) => {
            error!("Error getting file datapoint: {}", e.to_string());
            IoTDatapointFile::empty()
        }
    }
}
