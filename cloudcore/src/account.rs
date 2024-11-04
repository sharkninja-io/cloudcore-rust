#[cfg(feature = "library")]
mod user_account;
#[cfg(feature = "library")]
use self::user_account::{CreateAccountResponse, UserRequest};
#[cfg(feature = "library")]
use crate::ErrorUtil;
#[cfg(feature = "library")]
use crate::cloudcore::{ApplicationInfo, CloudCore};
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use serde::Serialize;
#[cfg(feature = "library")]
use tokio::time::sleep;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::time::Duration;
#[cfg(feature = "library")]
use crate::authentication::{CACHE_USER_DIR, CACHE_USER_SESSION_KEY};
#[cfg(feature = "library")]
use crate::cache::CacheInteract;

#[cfg(feature = "library")]
use crate::urls::{
    AUTHORIZATION_BEARER, AUTHORIZATION_HEADER, AYLA_CONFIRMATION_JSON, AYLA_PASSWORD_JSON,
    AYLA_UPDATE_EMAIL_JSON, AYLA_USER_JSON,
};

#[cfg(feature = "library")]
impl CloudCore {

    pub async fn create_account(
        &mut self,
        password: String,
        email: Option<String>,
        phone_number: Option<String>,
        email_template_id: Option<String>,
        email_subject: Option<String>,
        email_body_html: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Sleeping for 1 second so Ayla can't blame us for sending requests within '300ms'");
        sleep(Duration::from_secs(1)).await;
        #[derive(Serialize, Debug)]
        struct User {
            email: String,
            password: String,
            firstname: String,
            lastname: String,
            application: ApplicationInfo,
            country: Option<String>,
            city: Option<String>,
            street: Option<String>,
            zip: Option<String>,
            phone_country_code: Option<String>,
            phone_number: Option<String>,
            username: Option<String>,
            primary_contact: Option<String>,
        }
        #[derive(Serialize, Debug)]
        struct UserRequest {
            user: User,
        }
        let mut query: Vec<(&str, String)> = vec![];
        if email.is_some() {
            if let Some(template) = email_template_id {
                query.push(("email_template_id", template));
            } else {
                if let Some(subject) = email_subject {
                    query.push(("email_subject", subject));
                }
                if let Some(body_html) = email_body_html {
                    query.push(("email_body_html", body_html));
                }
            }
        }
        let mut phone_country_code = None;
        let mut phone = None;
        let mut username = None;
        let mut primary_contact = None;
        let mut use_dev = false;
        let email = match email {
            Some(email) => match email.strip_prefix("dev@") {
                Some(trimmed) => {
                    use_dev = true;
                    trimmed.to_string()
                },
                None => email
            },
            None => {
                primary_contact = Some("phone".to_string());
                phone = Some(phone_number.as_ref().unwrap().to_string());
                let un = phone_number.unwrap();
                phone_country_code = match un.strip_prefix("+1") {
                    Some(_) => Some("+1".to_string()),
                    None => match un.strip_prefix("+86") {
                        Some(_) => Some("+86".to_string()),
                        None => None
                    }
                };
                username = Some(un.clone());
                format!("cloudcoretest{}@gmail.com", un)
            }
        };
        self.set_ayla_region_environment(use_dev);
        let user_data = UserRequest {
            user: User {
                email,
                password,
                firstname: "FirstName".to_string(),
                lastname: "LastName".to_string(),
                application: self.session_params().app_info.clone(),
                country: None,
                city: None,
                street: None,
                zip: None,
                phone_country_code,
                phone_number: phone,
                username,
                primary_contact
            },
        };
        let mut url = String::from(&self.session_params().user_url);
        url.push_str(AYLA_USER_JSON);
        let response = self.client()
            .post(url)
            .query(&query)
            .json(&user_data)
            .send()
            .await?;
        if response.status().is_success() {
            let create_account_payload = response.json::<CreateAccountResponse>().await?;
            debug!("create account payload: {:#?}", create_account_payload);
            // If an error is thrown here it does need to propagate up because it just affects the cache
            let _ = self.cache.remove_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn confirm_account(&self, token: String) -> Result<(), Box<dyn Error>> {
        let query: Vec<(&str, String)> = vec![("confirmation_token", token.trim().to_string())];
        let mut url = String::from(&self.session_params().user_url);
        url.push_str(AYLA_CONFIRMATION_JSON);
        let response = self.client().put(url).query(&query).send().await?;
        if response.status().is_success() {
            let create_account_payload = response.json::<CreateAccountResponse>().await?;
            debug!("create account payload: {:#?}", create_account_payload);
            Ok(())
        } else {
            // Return any errors, since this could fail because
            // input is incorrect
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn send_confirmation_instructions(
        &self,
        email: Option<String>,
        phone_number: Option<String>,
        email_template_id: Option<String>,
        email_subject: Option<String>,
        email_body_html: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        if email.is_none() && phone_number.is_none() {
            return Err(Box::new(ErrorUtil::email_or_phone_number_missing_error()));
        }
        let mut username = None;
        let mut query: Vec<(&str, String)> = vec![];
        if email.is_some() {
            if let Some(template) = email_template_id {
                query.push(("user.email_template_id", template));
            } else {
                if let Some(subject) = email_subject {
                    query.push(("user.email_subject", subject));
                }
                if let Some(body_html) = email_body_html {
                    query.push(("user.email_body_html", body_html));
                }
            }
        } else {
            username = Some(phone_number.as_ref().unwrap().to_string());
        }
        let user_data = UserRequest::new(email, username, self.session_params().app_info.clone());
        let mut url = String::from(&self.session_params().user_url);
        url.push_str(AYLA_CONFIRMATION_JSON);
        let response = self.client()
            .post(url)
            .query(&query)
            .json(&user_data)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn delete_account(&mut self) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_USER_JSON);
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

        let client = self.client();

        let response = client
            .delete(url)
            .header(AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            self.user_session = None;
            // If an error is thrown here it does need to propagate up because it just affects the cache
            let _ = self.cache.remove_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn request_password_reset(
        &mut self,
        email: Option<String>,
        phone: Option<String>,
        email_template_id: Option<String>,
        email_subject: Option<String>,
        email_body_html: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        if email.is_none() && phone.is_none() {
            return Err(Box::new(ErrorUtil::email_or_phone_number_missing_error()));
        }
        let mut username = None;
        let mut query: Vec<(&str, String)> = vec![];
        let mut use_dev = false;
        if email.is_some() {
            if let Some(template) = email_template_id {
                query.push(("user.email_template_id", template));
            } else {
                if let Some(subject) = email_subject {
                    query.push(("user.email_subject", subject));
                }
                if let Some(body_html) = email_body_html {
                    query.push(("user.email_body_html", body_html));
                }
            }
        } else {
            username = Some(phone.as_ref().unwrap().to_string())
        }
        let email = match email {
            None => None,
            Some(email) => {
                match email.strip_prefix("dev@") {
                    Some(trimmed) => {
                        use_dev = true;
                        Some(trimmed.to_string())
                    },
                    None => Some(email)
                }
            }
        };
        self.set_ayla_region_environment(use_dev);
        let user_data = UserRequest::new(email, username, self.session_params().app_info.clone());
        debug!("User data for password request: {:#?}", user_data);
        let mut url = String::from(&self.session_params().user_url);
        url.push_str(AYLA_PASSWORD_JSON);
        let response = self.client()
            .post(url)
            .query(&query)
            .json(&user_data)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            // Return any errors, since this could fail because
            // input is incorrect
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn reset_password(
        &mut self,
        token: String,
        password: String,
        password_confirmation: String,
    ) -> Result<(), Box<dyn Error>> {
        if password != password_confirmation {
            return Err(Box::new(ErrorUtil::passwords_mismatch_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_PASSWORD_JSON);
        url.push_str(&endpoint);
        #[derive(Serialize, Debug)]
        struct PasswordUser {
            reset_password_token: String,
            password: String,
            password_confirmation: String,
        }
        #[derive(Serialize, Debug)]
        struct PasswordRequest {
            user: PasswordUser,
        }
        let pw_data = PasswordRequest {
            user: PasswordUser {
                reset_password_token: token.trim().to_string(),
                password,
                password_confirmation,
            },
        };
        let response = self.client()
            .put(url)
            .json(&pw_data)
            .send()
            .await?;
        if response.status().is_success() {
            self.user_session = None;
            // If an error is thrown here it does need to propagate up because it just affects the cache
            // Even though the user session is not used, it could still exist
            let _ = self.cache.remove_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
            Ok(())
        } else {
            // Return any errors, since this could fail because
            // input is incorrect
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn reset_password_for_current_user(
        &mut self,
        current_password: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_USER_JSON);
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);

        #[derive(Serialize, Debug)]
        struct PasswordUser {
            password: String,
            current_password: String,
        }
        #[derive(Serialize, Debug)]
        struct PasswordRequest {
            user: PasswordUser,
        }
        let pw_data = PasswordRequest {
            user: PasswordUser {
                current_password,
                password: new_password,
            },
        };

        let response = self.client()
            .put(url)
            .header(AUTHORIZATION_HEADER, auth_bearer)
            .json(&pw_data)
            .send()
            .await?;
        if response.status().is_success() {
            self.user_session = None;
            // If an error is thrown here it does need to propagate up because it just affects the cache
            // Even though the user session is not used, it could still exist
            let _ = self.cache.remove_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn update_email(&mut self, new_email: String) -> Result<(), Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }
        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(AYLA_UPDATE_EMAIL_JSON);
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();
        let auth_bearer = format!("{} {}", AUTHORIZATION_BEARER, token);
        let calculated_new_email = match new_email.strip_prefix("dev@") {
            Some(trimmed) => {
                trimmed.to_string()
            },
            None => new_email
        };
        let query: Vec<(&str, String)> = vec![("email", calculated_new_email)];
        let client = self.client();

        let response = client
            .put(url)
            .query(&query)
            .header(AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        if response.status().is_success() {
            // This invalidates the user session in Ayla, so remove it here as well
            self.user_session = None;
            // If an error is thrown here it does need to propagate up because it just affects the cache
            let _ = self.cache.remove_value(CACHE_USER_DIR.to_string(), CACHE_USER_SESSION_KEY.to_string());
            Ok(())
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }
}
