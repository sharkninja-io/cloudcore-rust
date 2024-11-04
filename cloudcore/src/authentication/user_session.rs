use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    access_token: String,
    refresh_token: String,
    auth_expiration_date: u64,
    auth_username: String,
    user_uuid: Option<String>,
    use_dev: bool
}

impl UserSession {
    pub fn new(
        access_token: String,
        refresh_token: String,
        auth_expiration_date: u64,
        auth_username: String,
        user_uuid: Option<String>,
        use_dev: bool
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            auth_expiration_date,
            auth_username,
            user_uuid,
            use_dev
        }
    }

    /// Get a reference to the user session's access token.
    pub fn access_token(&self) -> &str {
        self.access_token.as_ref()
    }
    /// Get a reference to the user session's refresh token.
    pub fn refresh_token(&self) -> &str {
        self.refresh_token.as_ref()
    }
    /// Get a reference to the user session's auth expiration date.
    pub fn auth_expiration_date(&self) -> u64 {
        self.auth_expiration_date
    }
    /// Get a reference to the user session's auth username.
    pub fn auth_username(&self) -> &str {
        self.auth_username.as_ref()
    }
    /// Get a reference to the user session's user uuid.
    pub fn user_uuid(&self) -> Option<&String> {
        self.user_uuid.as_ref()
    }
    /// Get the user session's use dev field.
    pub fn use_dev(&self) -> bool {
        self.use_dev
    }

    /// Set the user session's user uuid.
    pub fn set_user_uuid(&mut self, user_uuid: Option<String>) {
        self.user_uuid = user_uuid;
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u32,
    role: String,
}

impl LoginResponse {
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    pub fn expires_in(&self) -> u32 {
        self.expires_in.clone()
    }

    pub fn role(&self) -> String {
        self.role.clone()
    }
}
