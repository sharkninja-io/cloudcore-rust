use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoTDevice {
    key: Option<u32>,
    product_name: Option<String>,
    model: Option<String>,
    pub(crate) dsn: Option<String>,
    oem_model: Option<String>,
    sw_version: Option<String>,
    template_id: Option<u32>,
    mac: Option<String>,
    lan_ip: Option<String>,
    connected_at: Option<String>,
    lan_enabled: Option<bool>,
    has_properties: Option<bool>,
    connection_status: Option<String>,
    lat: Option<String>,
    lng: Option<String>,
    device_type: Option<String>,
}

impl IoTDevice {
    pub fn new(
        key: Option<u32>,
        product_name: Option<String>,
        model: Option<String>,
        dsn: Option<String>,
        oem_model: Option<String>,
        sw_version: Option<String>,
        template_id: Option<u32>,
        mac: Option<String>,
        lan_ip: Option<String>,
        connected_at: Option<String>,
        lan_enabled: Option<bool>,
        has_properties: Option<bool>,
        connection_status: Option<String>,
        lat: Option<String>,
        lng: Option<String>,
        device_type: Option<String>,
    ) -> Self {
        Self {
            key,
            product_name,
            model,
            dsn,
            oem_model,
            sw_version,
            template_id,
            mac,
            lan_ip,
            connected_at,
            lan_enabled,
            has_properties,
            connection_status,
            lat,
            lng,
            device_type,
        }
    }

    /// Getters
    pub fn id(&self) -> Option<u32> {
        self.key
    }
    pub fn product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
    }
    pub fn model(&self) -> Option<&String> {
        self.model.as_ref()
    }
    pub fn dsn(&self) -> Option<&String> {
        self.dsn.as_ref()
    }
    pub fn oem_model(&self) -> Option<&String> {
        self.oem_model.as_ref()
    }
    pub fn sw_version(&self) -> Option<&String> {
        self.sw_version.as_ref()
    }
    pub fn template_id(&self) -> Option<u32> {
        self.template_id
    }
    pub fn mac(&self) -> Option<&String> {
        self.mac.as_ref()
    }
    pub fn lan_ip(&self) -> Option<&String> {
        self.lan_ip.as_ref()
    }
    pub fn connected_at(&self) -> Option<&String> {
        self.connected_at.as_ref()
    }
    pub fn lan_enabled(&self) -> Option<bool> {
        self.lan_enabled
    }
    pub fn has_properties(&self) -> Option<bool> {
        self.has_properties
    }
    pub fn connection_status(&self) -> Option<&String> {
        self.connection_status.as_ref()
    }
    pub fn lat(&self) -> Option<&String> {
        self.lat.as_ref()
    }
    pub fn lng(&self) -> Option<&String> {
        self.lng.as_ref()
    }
    pub fn device_type(&self) -> Option<&String> {
        self.device_type.as_ref()
    }
}