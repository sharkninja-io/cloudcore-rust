mod user_session;
pub use self::user_session::{LoginResponse, UserSession};

#[cfg(feature = "library")]
mod user_profile;
#[cfg(feature = "library")]
pub use self::user_profile::UserProfile;
#[cfg(feature = "library")]
use crate::cloudcore::{ApplicationInfo, CloudCore};
#[cfg(feature = "library")]
use crate::ErrorUtil;
#[cfg(feature = "library")]
use crate::urls::{
    AUTHORIZATION_BEARER, AUTHORIZATION_HEADER, AYLA_REFRESH_TOKEN_JSON, AYLA_SIGN_IN_JSON,
    AYLA_SIGN_OUT_JSON, AYLA_USER_PROFILE_JSON,
};
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use serde::Serialize;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::time::SystemTime;
#[cfg(feature = "library")]
use crate::cache::CacheDataValue;
#[cfg(feature = "library")]
use crate::cache::CacheDir;
#[cfg(feature = "library")]
use crate::cache::CacheInteract;
#[cfg(feature = "library")]
use crate::cloudcore::SELECTED_REGION_CACHE_KEY;

pub static CACHE_USER_DIR: &str = "user";
pub static CACHE_USER_SESSION_KEY: &str = "session";

#[cfg(feature = "library")]
impl CloudCore {
    pub async fn login(
        &mut self,
        email: Option<String>,
        phone: Option<String>,
        password: String,
    ) -> Result<(), Box<dyn Error>> {
        if email.is_none() && phone.is_none() {
            return Err(Box::new(ErrorUtil::email_or_phone_number_missing_error()));
        }
        // local structs for login request and response
        #[derive(Serialize, Debug)]
        struct UserPayload {
            email: String,
            password: String,
            application: ApplicationInfo,
        }

        #[derive(Serialize, Debug)]
        struct LoginRequest {
            user: UserPayload,
        }
        let mut use_dev = false;
        let calculated_email: String;
        if let Some(email) = email {
            calculated_email = match email.strip_prefix("dev@") {
                Some(trimmed) => {
                    use_dev = true;
                    trimmed.to_string()
                },
                None => email
            };
        } else {
            calculated_email = phone.unwrap()
        }
        // This sets the session parameters based on the country stored in cache and if the environment should be dev or not
        self.set_ayla_region_environment(use_dev);
        let app_secret = String::from(&self.session_params().app_info.app_secret);
        let app_id = String::from(&self.session_params().app_info.app_id);
        let post_data = LoginRequest {
            user: UserPayload {
                email: calculated_email,
                password,
                application: ApplicationInfo { app_id, app_secret },
            },
        };
        debug!("login request data: {:#?}", post_data);
        let mut url = String::from(&self.session_params().user_url);
        url.push_str(AYLA_SIGN_IN_JSON);
        let response = self.client()
            .post(url)
            .json(&post_data)
            .send()
            .await?;
        if !response.status().is_success() {
            // Return any errors, since this could fail because
            // input is incorrect
            let error_payload = response.text().await?;
            return Err(Box::new(ErrorUtil::login_error(error_payload)))
        }
        let login_payload = response.json::<LoginResponse>().await?;
        debug!("login payload: {:#?}", login_payload);

        let expire_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + login_payload.expires_in() as u64;
        let mut user_session = UserSession::new(
            login_payload.access_token(),
            login_payload.refresh_token(),
            expire_time,
            post_data.user.email,
            None,
            use_dev
        );

        self.user_session = Some(user_session.clone());

        let user_profile = self.get_user_profile().await?;
        let uuid = user_profile.uuid().to_owned();

        user_session.set_user_uuid(Some(uuid));

        self.user_session = Some(user_session.clone());

        if let Some(err) = self.cache.set_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string(), self.user_session.clone()).err() {
            error!("Error saving user session to cache: {}", err.to_string());
        };

        Ok(())
    }

    // TODO: Just return an Option<UserSession> instead of a result
    pub fn get_session(&self) -> Result<UserSession, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        Ok(self.user_session.clone().unwrap())
    }

    pub fn set_session(&mut self, user_session: UserSession) {
        self.user_session = Some(user_session.clone());
        if let Some(err) = self.cache.set_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string(), self.user_session.clone()).err() {
            error!("Error saving user session to cache: {}", err.to_string());
        };
    }

    pub async fn refresh_session(&mut self) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let use_dev = self.user_session.as_ref().unwrap().use_dev();
        // This sets the session parameters based on the country stored in cache and if the environment should be dev or not
        self.set_ayla_region_environment(use_dev);
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_REFRESH_TOKEN_JSON);
        url.push_str(&endpoint);
        #[derive(Serialize, Debug)]
        struct UserRefresh {
            refresh_token: String,
        }
        #[derive(Serialize, Debug)]
        struct UserRefreshRequest {
            user: UserRefresh,
        }
        let refresh_data = UserRefreshRequest {
            user: UserRefresh {
                refresh_token: self.user_session.as_ref().unwrap().refresh_token().to_string(),
            },
        };
        let response = self.client()
            .post(url)
            .json(&refresh_data)
            .send()
            .await?;
        if response.status().is_success() {
            let refresh_payload = response.json::<LoginResponse>().await?;
            debug!("refresh payload: {:#?}", refresh_payload);
            let current_user_session = self.user_session.as_ref().unwrap();
            let expire_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + refresh_payload.expires_in() as u64;
            let user_session = UserSession::new(
                refresh_payload.access_token(),
                refresh_payload.refresh_token(),
                expire_time,
                current_user_session.auth_username().to_string(),
                Some(current_user_session.user_uuid().unwrap().to_string()),
                use_dev
            );
            self.user_session = Some(user_session.clone());
            if let Some(err) = self.cache.set_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string(), self.user_session.clone()).err() {
                error!("Error saving user session to cache: {}", err.to_string());
            };
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub fn logged_in(&self) -> bool {
        match self.user_session.as_ref() {
            Some(sess) => {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if sess.auth_expiration_date() > now {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub async fn logout(&mut self) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_SIGN_OUT_JSON);
        url.push_str(&endpoint);
        #[derive(Serialize, Debug)]
        struct User {
            access_token: String,
        }
        #[derive(Serialize, Debug)]
        struct LogoutRequest {
            user: User,
        }
        let request = LogoutRequest {
            user: User {
                access_token: self.user_session.as_ref().unwrap().access_token().to_string(),
            },
        };
        // For logout, don't really care if we succeed. Just set self.session to None
        let response = self.client()
            .post(url)
            .json(&request)
            .send()
            .await?;
        if response.status().is_success() {
            debug!("Logged out");
        } else {
            error!("Could not log user out: {}", response.text().await?);
        }
        self.user_session = None;
        let country_region_selection = self.cache.get_value(CACHE_USER_DIR.to_string(), SELECTED_REGION_CACHE_KEY.to_string()).ok();
        self.cache.delete();
        if let Some(err) = self.cache.make_dir_for_child(CACHE_USER_DIR).err() {
            error!("Error saving user session to cache: {}", err.to_string());
        } else {
            if let Some(cached_country) = country_region_selection {
                match cached_country {
                    CacheDataValue::StringValue(string) => {
                        let _ = self.cache.set_value(CACHE_USER_DIR.to_string(), SELECTED_REGION_CACHE_KEY.to_string(), string);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub async fn get_user_profile(&self) -> Result<UserProfile, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_USER_PROFILE_JSON);
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

        let client = self.client();

        let response = client
            .get(url)
            .header(AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            let user_profile_payload = response.json::<UserProfile>().await?;
            Ok(user_profile_payload)
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }
}