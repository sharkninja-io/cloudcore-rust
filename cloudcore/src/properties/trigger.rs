use serde::{Deserialize, Serialize};

pub static NOTIFICATION_TYPE_ERROR: i32 = 300;
pub static ERROR_NOTIFICATION_PROPERTY_NAME: &str = "Get_ERROR_Code";
pub static ERROR_NOTIFICATION_PROPERTY_TYPE: &str = "integer";
pub static ERROR_NOTIFICATION_TRIGGER_COMPARE_TYPE: &str = "==";
pub static ERROR_NOTIFICATION_TRIGGER_TYPE: &str = "compare_absolute";
pub static ERROR_NOTIFICATION_TRIGGER_ACTIVE_DEFAULT: bool = true;

pub static TRIGGER_APP_NAME_FCM: &str = "push_android_fcm";
pub static TRIGGER_APP_NAME_BAIDU: &str = "push_baidu";
pub static TRIGGER_APP_NAME_IOS: &str = "push_ios";
pub static TRIGGER_APP_NAME_GOOGLE: &str = "push_android"; // TODO: USED?

#[derive(Serialize, Debug)]
pub struct AndroidDeviceRequest {
    pub android_device_id: String,
}

#[derive(Serialize, Debug)]
pub struct IOSDeviceRequest {
    pub ios_device_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoTTrigger {
    pub key: u32,
    property_key: u32,
    property_name: String,
    trigger_type: String,
    compare_type: String,
    pub value: String,
    pub trigger_apps: Vec<IoTTriggerApp>,
    device_nickname: Option<String>,
    property_nickname: String,
    base_type: String,
    period: String,
    asset: bool,
    active: bool,
    user_uuid: String,
    user_id: u32,
    trigger_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoTTriggerApp {
    pub key: u32,
    pub name: String,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub trigger_key: u32,
    pub contact_id: Option<String>,
    pub repeat_freq: Option<u32>,
    pub push_sound: Option<String>,
    pub push_mdata: Option<String>,
    pub email_template_id: Option<String>,
    pub email_subject: Option<String>,
    pub email_body_html: Option<String>,
    pub param1: Option<String>,
    pub param2: Option<String>,
    pub param3: Option<String>,
    pub param4: Option<String>,
    pub param5: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct TriggerRequest {
    device_nickname: String,
    property_nickname: String,
    compare_type: String,
    trigger_type: String,
    value: String,
    active: bool,
    base_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TriggerAppRequest {
    name: String,
    nickname: Option<String>,
    repeat_freq: Option<u32>,
    // repeat interval in seconds
    pub param1: Option<String>,
    param2: Option<String>,
    pub param3: Option<String>,
    param4: Option<String>,
    param5: Option<String>,
    contact_id: Option<String>,
    push_sound: Option<String>,
    push_mdata: Option<String>,
    email_template_id: Option<String>,
    email_subject: Option<String>,
    email_body_html: Option<String>,
    requires_acceptance: Option<bool>,
}

impl IoTTrigger {
    pub fn new(
        key: u32,
        property_key: u32,
        property_name: String,
        trigger_type: String,
        compare_type: String,
        value: String,
        trigger_apps: Vec<IoTTriggerApp>,
        device_nickname: Option<String>,
        property_nickname: String,
        base_type: String,
        period: String,
        asset: bool,
        active: bool,
        user_uuid: String,
        user_id: u32,
        trigger_at: Option<String>,
    ) -> Self {
        Self {
            key,
            property_key,
            property_name,
            trigger_type,
            compare_type,
            value,
            trigger_apps,
            device_nickname,
            property_nickname,
            base_type,
            period,
            asset,
            active,
            user_uuid,
            user_id,
            trigger_at,
        }
    }
    pub fn add_trigger_app(&mut self, trigger_app: IoTTriggerApp) {
        self.trigger_apps.push(trigger_app)
    }
    pub fn empty() -> Self {
        Self {
            key: 0,
            property_key: 0,
            property_name: "".to_string(),
            trigger_type: "".to_string(),
            compare_type: "".to_string(),
            value: "".to_string(),
            trigger_apps: vec![],
            device_nickname: None,
            property_nickname: "".to_string(),
            base_type: "".to_string(),
            period: "".to_string(),
            asset: false,
            active: false,
            user_uuid: "".to_string(),
            user_id: 0,
            trigger_at: None,
        }
    }

    pub fn get_trigger_apps_for_device(&self, device_id: String) -> Vec<IoTTriggerApp> {
        self.trigger_apps.iter().filter(|ta| ta.has_device(device_id.to_string()))
            .map(|ta| return ta.clone())
            .collect()
    }
}

impl IoTTriggerApp {
    pub fn new(
        key: u32,
        name: String,
        username: Option<String>,
        nickname: Option<String>,
        trigger_key: u32,
        contact_id: Option<String>,
        repeat_freq: Option<u32>,
        push_sound: Option<String>,
        push_mdata: Option<String>,
        email_template_id: Option<String>,
        email_subject: Option<String>,
        email_body_html: Option<String>,
        param1: Option<String>,
        param2: Option<String>,
        param3: Option<String>,
        param4: Option<String>,
        param5: Option<String>,
    ) -> Self {
        Self {
            key,
            name,
            username,
            nickname,
            trigger_key,
            contact_id,
            repeat_freq,
            push_sound,
            push_mdata,
            email_template_id,
            email_subject,
            email_body_html,
            param1,
            param2,
            param3,
            param4,
            param5,
        }
    }

    pub fn has_device(&self, device_id: String) -> bool {
        if let Some(meta) = &self.push_mdata {
            !device_id.is_empty() && meta.as_str().contains(device_id.as_str())
        } else {
            false
        }
    }

    pub fn update_message(&mut self, message: String) {
        self.param3 = Some(message)
    }

    pub fn update_registration_id(&mut self, registration_id: String) {
        self.param1 = Some(registration_id)
    }
}

impl TriggerRequest {
    pub fn new(
        device_nickname: String,
        property_nickname: String,
        compare_type: String,
        trigger_type: String,
        value: String,
        active: bool,
        base_type: String,
    ) -> Self {
        Self {
            device_nickname,
            property_nickname,
            compare_type,
            trigger_type,
            value,
            active,
            base_type,
        }
    }

    pub fn new_error_request(
        device_nickname: String,
        error_code: String,
    ) -> Self {
        Self {
            device_nickname,
            property_nickname: ERROR_NOTIFICATION_PROPERTY_NAME.to_string(),
            compare_type: ERROR_NOTIFICATION_TRIGGER_COMPARE_TYPE.to_string(),
            trigger_type: ERROR_NOTIFICATION_TRIGGER_TYPE.to_string(),
            value: error_code,
            active: ERROR_NOTIFICATION_TRIGGER_ACTIVE_DEFAULT,
            base_type: ERROR_NOTIFICATION_PROPERTY_TYPE.to_string(),
        }
    }
}

impl TriggerAppRequest {
    pub fn new(
        name: String,
        nickname: Option<String>,
        repeat_freq: Option<u32>,
        // repeat interval in seconds
        param1: Option<String>,
        param2: Option<String>,
        param3: Option<String>,
        param4: Option<String>,
        param5: Option<String>,
        contact_id: Option<String>,
        push_sound: Option<String>,
        push_mdata: Option<String>,
        email_template_id: Option<String>,
        email_subject: Option<String>,
        email_body_html: Option<String>,
        requires_acceptance: Option<bool>,
    ) -> Self {
        Self {
            name,
            nickname,
            repeat_freq,
            param1,
            param2,
            param3,
            param4,
            param5,
            contact_id,
            push_sound,
            push_mdata,
            email_template_id,
            email_subject,
            email_body_html,
            requires_acceptance,
        }
    }
    pub fn baidu_request(
        application_id: String,
        channel_id: String,
        baidu_msg: String,
        push_sound: Option<String>,
        push_metadata: String,
    ) -> Self {
        Self {
            name: TRIGGER_APP_NAME_BAIDU.to_string(),
            nickname: None,
            repeat_freq: None,
            param1: Some(application_id),
            param2: Some(channel_id),
            param3: Some(baidu_msg),
            param4: None,
            param5: None,
            push_sound,
            push_mdata: Some(push_metadata),
            email_template_id: None,
            email_subject: None,
            email_body_html: None,
            contact_id: None,
            requires_acceptance: None,
        }
    }
    pub fn fcm_request(
        registration_id: String,
        application_id: String,
        message: String,
        push_sound: Option<String>,
        push_metadata: String,
    ) -> Self {
        Self {
            name: TRIGGER_APP_NAME_FCM.to_string(),
            nickname: None,
            repeat_freq: None,
            param1: Some(registration_id),
            param2: Some(application_id),
            param3: Some(message),
            param4: None,
            param5: None,
            push_sound,
            push_mdata: Some(push_metadata),
            email_template_id: None,
            email_subject: None,
            email_body_html: None,
            contact_id: None,
            requires_acceptance: None,
        }
    }
    pub fn ios_request(
        registration_id: String,
        application_id: String,
        message: String,
        push_sound: Option<String>,
        push_metadata: String,
    ) -> Self {
        Self {
            name: TRIGGER_APP_NAME_IOS.to_string(),
            nickname: None,
            repeat_freq: None,
            param1: Some(registration_id),
            param2: Some(application_id),
            param3: Some(message),
            param4: None,
            param5: None,
            push_sound,
            push_mdata: Some(push_metadata),
            email_template_id: None,
            email_subject: None,
            email_body_html: None,
            contact_id: None,
            requires_acceptance: None,
        }
    }

    pub fn from_trigger_app(trigger_app: IoTTriggerApp) -> Self {
        Self {
            name: trigger_app.name,
            nickname: trigger_app.nickname,
            repeat_freq: trigger_app.repeat_freq,
            param1: trigger_app.param1,
            param2: trigger_app.param2,
            param3: trigger_app.param3,
            param4: trigger_app.param4,
            param5: trigger_app.param5,
            push_sound: trigger_app.push_sound,
            push_mdata: trigger_app.push_mdata,
            email_template_id: trigger_app.email_template_id,
            email_subject: trigger_app.email_subject,
            email_body_html: trigger_app.email_body_html,
            contact_id: trigger_app.contact_id,
            requires_acceptance: None
        }
    }
}