#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use reqwest::{Method, RequestBuilder, Response};
#[cfg(feature = "library")]
use reqwest::header::{ACCEPT, CONTENT_TYPE};
#[cfg(feature = "library")]
use serde::Serialize;

#[cfg(feature = "library")]
use crate::{CloudCore, urls};
#[cfg(feature = "library")]
use crate::ErrorUtil;

#[cfg(feature = "library")]
pub enum CloudCoreBaseURL {
    USER,
    DEVICE,
}

#[cfg(feature = "library")]
pub struct CloudCoreRequest<T: Serialize + Sized> {
    pub base_url: CloudCoreBaseURL,
    pub endpoint: String,
    pub method: Method,
    pub requires_auth: bool,
    pub body: Option<T>,
}

pub static ERROR_PATTERN_TOO_MANY_INSTANCES: &str = "too many instances had been created already";

#[cfg(feature = "library")]
impl CloudCore {
    async fn get_access_token(
        &self,
        cc: &mut CloudCore,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, Box<dyn Error>> {
        if cc.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        if !cc.logged_in() {
            let refresh_result = cc.refresh_session().await;
            match refresh_result {
                Ok(_) => println!("Token refreshed for API call."),
                Err(_) => return Err("Refresh token for API call failed.".into())
            }
        }
        let token = cc.user_session.as_ref().unwrap().access_token();
        Ok(
            request.header(urls::AUTHORIZATION_HEADER,
                           format!("{} {}", urls::AUTHORIZATION_BEARER, token))
        )
    }
    pub async fn send_request<T: Serialize>(
        &self,
        cloudcore_request: CloudCoreRequest<T>,
    ) -> Result<Response, Box<dyn Error>> {
        let cc = CloudCore::shared();
        let params = cc.session_params();
        let base_url = match cloudcore_request.base_url {
            CloudCoreBaseURL::DEVICE => String::from(params.device_url.to_string()),
            CloudCoreBaseURL::USER => String::from(params.user_url.to_string())
        };
        let needs_auth = cloudcore_request.requires_auth;
        let mut request = self.create_request(base_url, cloudcore_request)?;
        if needs_auth {
            request = self.get_access_token(cc, request).await?;
        }
        self.get_response(request).await
    }

    fn create_request<T: Serialize>(
        &self,
        mut url: String,
        cloudcore_request: CloudCoreRequest<T>,
    ) -> Result<RequestBuilder, Box<dyn Error>> {
        url.push_str(&cloudcore_request.endpoint);
        let client = self.client();
        let request = match cloudcore_request.method {
            Method::GET => {
                client.get(url)
                    .header(ACCEPT, "application/json")
            }
            Method::PUT => {
                client.put(url)
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .json(&cloudcore_request.body.unwrap())
            }
            Method::POST => {
                client.post(url)
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .json(&cloudcore_request.body.unwrap())
            }
            Method::DELETE => {
                client.delete(url)
            }
            _ => return Err(Box::new(ErrorUtil::invalid_method()))
        };
        Ok(request)
    }

    async fn get_response(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<Response, Box<dyn Error>> {
        let response = request_builder.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let error_payload = response.text().await?;
            let error = match status.as_u16() {
                422 => {
                    if error_payload.contains(ERROR_PATTERN_TOO_MANY_INSTANCES) {
                        ErrorUtil::too_many_instances_error(error_payload)
                    } else {
                        ErrorUtil::server_error(error_payload)
                    }
                },
                _ => ErrorUtil::server_error(error_payload)
            };
            Err(Box::new(error))
        }
    }
}