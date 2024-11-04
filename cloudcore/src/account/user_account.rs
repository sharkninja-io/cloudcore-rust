use crate::cloudcore::ApplicationInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CreateAccountResponse {
    admin: bool,
    alternate_email: Option<String>,
    approved: bool,
    created_at: String,
    email: String,
    firstname: String,
    id: u32,
    lastname: String,
    oem_approved: bool,
    origin_oem_id: u32,
    phone_number: Option<String>,
    primary_contact: String,
    terms_accepted: bool,
    terms_accepted_at: String,
    updated_at: String,
    username: String,
    uuid: String,
}

#[derive(Serialize, Debug)]
pub struct UserRequest {
    user: User,
}

#[derive(Serialize, Debug)]
pub struct User {
    email: Option<String>,
    username: Option<String>,
    contact_option: Option<String>,
    application: ApplicationInfo,
}

#[cfg(feature = "library")]
impl UserRequest {
    pub fn new(email: Option<String>, username: Option<String>, application: ApplicationInfo) -> Self {
        let contact_option = if username.is_some() {
            Some("phone".to_string())
        } else {
            None
        };
        Self {
            user: User { email, username, contact_option, application },
        }
    }
}
