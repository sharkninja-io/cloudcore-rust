use serde::{Deserialize, Serialize};
use crate::properties::value::IoTPropertyValue;

pub static PROPS_PATH_PARAMS_DSN: &str = "<dsn>";
pub static PROPS_PATH_PARAMS_PROP_NAME: &str = "<prop_name>";
pub static PROPS_PATH_PARAMS_DATAPOINT_ID: &str = "<datapoint_id>";

#[derive(Debug, Serialize, Deserialize)]
pub struct IoTProperty {
    r#type: String,
    name: String,
    base_type: String,
    read_only: bool,
    direction: String,
    scope: String,
    data_updated_at: Option<String>,
    key: Option<u32>,
    device_key: Option<u32>,
    product_name: Option<String>,
    track_only_changes: bool,
    display_name: String,
    host_sw_version: bool,
    time_series: bool,
    derived: bool,
    app_type: Option<String>,
    recipe: Option<String>,
    value: Option<IoTPropertyValue>,
    ack_enabled: bool,
    ack_status: Option<String>,
    ack_message: Option<String>,
    acked_at: Option<String>,
}

impl IoTProperty {
    pub fn new(
        r#type: String,
        name: String,
        base_type: String,
        read_only: bool,
        direction: String,
        scope: String,
        data_updated_at: Option<String>,
        key: Option<u32>,
        device_key: Option<u32>,
        product_name: String,
        track_only_changes: bool,
        display_name: String,
        host_sw_version: bool,
        time_series: bool,
        derived: bool,
        app_type: Option<String>,
        recipe: Option<String>,
        value: Option<IoTPropertyValue>,
        ack_enabled: bool,
        ack_status: Option<String>,
        ack_message: Option<String>,
        acked_at: Option<String>,
    ) -> Self {
        Self {
            r#type,
            name,
            base_type,
            read_only,
            direction,
            scope,
            data_updated_at,
            key,
            device_key,
            product_name: Some(product_name),
            track_only_changes,
            display_name,
            host_sw_version,
            time_series,
            derived,
            app_type,
            recipe,
            value,
            ack_enabled,
            ack_status,
            ack_message,
            acked_at,
        }
    }

    /// Getters
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base_type(&self) -> &str {
        &self.base_type
    }
    pub fn read_only(&self) -> bool {
        self.read_only
    }
    pub fn direction(&self) -> &str {
        &self.direction
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn data_updated_at(&self) -> Option<&String> {
        self.data_updated_at.as_ref()
    }
    pub fn key(&self) -> Option<u32> {
        self.key
    }
    pub fn device_key(&self) -> Option<u32> {
        self.key
    }
    pub fn product_name(&self) -> &str {
        // Always just return an empty string if value is None
        match self.product_name.as_ref() {
            Some(name) => name.as_ref(),
            None => ""
        }
    }
    pub fn track_only_changes(&self) -> bool {
        self.track_only_changes
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn host_sw_version(&self) -> bool {
        self.host_sw_version
    }
    pub fn time_series(&self) -> bool {
        self.time_series
    }
    pub fn derived(&self) -> bool {
        self.derived
    }
    pub fn app_type(&self) -> Option<&String> {
        self.app_type.as_ref()
    }
    pub fn recipe(&self) -> Option<&String> {
        self.recipe.as_ref()
    }
    pub fn value(&self) -> Option<&IoTPropertyValue> {
        self.value.as_ref()
    }
    pub fn ack_enabled(&self) -> bool {
        self.ack_enabled
    }
    pub fn ack_status(&self) -> Option<&String> {
        self.ack_status.as_ref()
    }
    pub fn ack_message(&self) -> Option<&String> {
        self.ack_message.as_ref()
    }
    pub fn acked_at(&self) -> Option<&String> {
        self.acked_at.as_ref()
    }
}
