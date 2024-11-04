use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WifiNetwork {
    bars: Option<u32>,
    bssid: Option<String>,
    chan: Option<u32>,
    security: Option<String>,
    signal: Option<i32>,
    ssid: Option<String>,
    r#type: Option<String>,
    password: Option<String>,
}

impl WifiNetwork {

    pub fn new(
        bars: Option<u32>,
        bssid: Option<String>,
        chan: Option<u32>,
        security: Option<String>,
        signal: Option<i32>,
        ssid: Option<String>,
        r#type: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            bars,
            bssid,
            chan,
            security,
            signal,
            ssid,
            r#type,
            password
        }
    }

    /// Getters
    pub fn bars(&self) -> Option<u32> {
        self.bars
    }
    pub fn bssid(&self) -> Option<&String> {
        self.bssid.as_ref()
    }
    pub fn chan(&self) -> Option<u32> {
        self.chan
    }
    pub fn security(&self) -> Option<&String> {
        self.security.as_ref()
    }
    pub fn signal(&self) -> Option<i32> {
        self.signal
    }
    pub fn ssid(&self) -> Option<&String> {
        self.ssid.as_ref()
    }
    pub fn r#type(&self) -> Option<&String> {
        self.r#type.as_ref()
    }
    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Setters
    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }
}