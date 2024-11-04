use serde::{Deserialize, Serialize};
use crate::properties::value::IoTPropertyValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataPointResponse {
    pub(crate) datapoint: IoTDatapoint
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IoTDatapoint {
    // Not sure if these will ever be useful
    //generated_at: Option<String>,
    //generated_from: Option<String>,
    //created_at_from_device: Option<String>,
    value: IoTPropertyValue,
    metadata: IoTDatapointMetadata,
    updated_at: Option<String>,
    created_at: Option<String>,
    echo: Option<bool>,
}

impl IoTDatapoint {
    pub fn new(
        value: IoTPropertyValue,
        metadata: IoTDatapointMetadata,
        updated_at: Option<String>,
        created_at: Option<String>,
        echo: Option<bool>,
    ) -> Self {
        Self {
            value,
            metadata,
            updated_at,
            created_at,
            echo,
        }
    }

    /// Get a reference to the iot datapoint's value.
    pub fn value(&self) -> &IoTPropertyValue {
        &self.value
    }

    /// Get a reference to the iot datapoint's metadata.
    pub fn metadata(&self) -> &IoTDatapointMetadata {
        &self.metadata
    }

    /// Get a reference to the iot datapoint's updated_at.
    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }

    /// Get a reference to the iot datapoint's updated_at.
    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    /// Get a reference to the iot datapoint file's echo.
    pub fn echo(&self) -> Option<bool> {
        self.echo
    }

    /// Set the iot datapoint's value.
    pub fn set_value(&mut self, value: IoTPropertyValue) {
        self.value = value;
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoTDatapointMetadata {
    userUUID: Option<String>,
}

#[allow(non_snake_case)]
impl IoTDatapointMetadata {
    pub fn new(userUUID: String) -> Self {
        Self { userUUID: Some(userUUID) }
    }

    /// Get a reference to the iot datapoint metadata's user uuid.
    pub fn user_uuid(&self) -> Option<&String> {
        self.userUUID.as_ref()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct IoTDatapointFile {
    updated_at: String,
    created_at: String,
    echo: bool,
    closed: bool,
    generated_at: Option<String>,
    generated_from: Option<String>,
    value: String,
    created_at_from_device: Option<String>,
    file: String,
    // Added once the file has been downloaded to the device
    local_file: Option<String>,
}

impl IoTDatapointFile {

    pub fn empty() -> Self {
        Self {
            updated_at: "".to_string(),
            created_at: "".to_string(),
            echo: false,
            closed: false,
            generated_at: None,
            generated_from: None,
            value: "".to_string(),
            created_at_from_device: None,
            file: "".to_string(),
            local_file: None
        }
    }

    /// Get a reference to the iot datapoint file's updated at.
    pub fn updated_at(&self) -> &str {
        self.updated_at.as_ref()
    }

    /// Get a reference to the iot datapoint file's created at.
    pub fn created_at(&self) -> &str {
        self.created_at.as_ref()
    }

    /// Get a reference to the iot datapoint file's echo.
    pub fn echo(&self) -> bool {
        self.echo
    }

    /// Get a reference to the iot datapoint file's closed.
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Get a reference to the iot datapoint file's generated at.
    pub fn generated_at(&self) -> Option<&String> {
        self.generated_at.as_ref()
    }

    /// Get a reference to the iot datapoint file's generated from.
    pub fn generated_from(&self) -> Option<&String> {
        self.generated_from.as_ref()
    }

    /// Get a reference to the iot datapoint file's value.
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }

    /// Get a reference to the iot datapoint file's created at from device.
    pub fn created_at_from_device(&self) -> Option<&String> {
        self.created_at_from_device.as_ref()
    }

    /// Get a reference to the iot datapoint file's file.
    pub fn file(&self) -> &str {
        self.file.as_ref()
    }

    /// Get a reference to the iot datapoint file's local file.
    pub fn local_file(&self) -> Option<&String> {
        self.local_file.as_ref()
    }

    /// Set local file after downloading
    pub fn set_local_file(&mut self, local_file: String) {
        self.local_file = Some(local_file);
    }
}

#[derive(Debug, Clone)]
pub struct IoTDatapointMessage {
    pub metadata: IoTDatapointMetadata,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub echo: Option<bool>,
    pub local_file: String,
}

impl IoTDatapointMessage {
    pub fn new(datapoint: IoTDatapoint, local_file: String) -> Self {
        Self {
            metadata: datapoint.metadata,
            updated_at: datapoint.updated_at,
            created_at: datapoint.created_at,
            echo: datapoint.echo,
            local_file,
        }
    }

    pub fn empty() -> Self {
        Self {
            metadata: IoTDatapointMetadata::new("".to_string()),
            updated_at: None,
            created_at: None,
            echo: None,
            local_file: "".to_string()
        }
    }
}
