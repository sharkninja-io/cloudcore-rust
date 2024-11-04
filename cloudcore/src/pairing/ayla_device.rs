#[cfg(feature = "signatures")]
#[derive(Debug, Clone)]
pub struct AylaDeviceInfo {
    device_url: String,
    ip_address: Option<String>,
    dsn: Option<String>,
    setup_token: Option<String>,
}

#[cfg(feature = "library")]
impl AylaDeviceInfo {
    pub fn new(device_url: String) -> Self {
        Self {
            device_url,
            ip_address: None,
            dsn: None,
            setup_token: None
        }
    }

    /// Getters
    pub fn device_url(&self) -> &String {
        &self.device_url
    }
    pub fn ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }
    pub fn dsn(&self) -> Option<&String> {
        self.dsn.as_ref()
    }
    pub fn setup_token(&self) -> Option<&String> {
        self.setup_token.as_ref()
    }

    /// Setters
    pub fn set_device_url(&mut self, device_url: String) {
        self.device_url = device_url;
    }
    pub fn set_ip_address(&mut self, ip_address: Option<String>) {
        self.ip_address = ip_address;
    }
    pub fn set_dsn(&mut self, dsn: Option<String>) {
        self.dsn = dsn;
    }
    pub fn set_setup_token(&mut self, setup_token: Option<String>) {
        self.setup_token = setup_token;
    }
}