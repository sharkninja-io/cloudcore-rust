use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub user_uuid: String,
    pub id: String,
    pub dsn: String,
    pub created_at: String,
    pub datapoint_created_at: String,
    pub read: bool,
    pub deleted: bool,
    pub notification_type: i32,
    pub notification_subtype: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationSubscription {
    pub notification_type: i32,
    pub notification_subtype: i32,
    pub subscribed: bool,
}

#[derive(PartialEq, Eq)]
pub enum NotificationService {
    PushAndroidFcm,
    PushAndroidBaidu,
    PushiOS
}

impl FromStr for NotificationService {
    type Err = ();
    fn from_str(input: &str) -> Result<NotificationService, Self::Err> {
        match input {
            "push_android_fcm" => Ok(NotificationService::PushAndroidFcm),
            "push_android_baidu" => Ok(NotificationService::PushAndroidBaidu),
            "push_ios" =>  Ok(NotificationService::PushiOS),
            _ => Err(()),
        }
    }
}

impl Notification {
    pub const MAX_AGE_DAYS: i64 = 7;

    pub fn matches(a: &Notification, b: &Notification) -> bool {
        a.datapoint_created_at == b.datapoint_created_at
            && a.notification_subtype == b.notification_subtype
            && a.notification_type == b.notification_type
            && a.dsn == b.dsn
    }

    pub fn mark_as_read(notification: &mut Notification) {
        notification.read = true;
    }

    pub fn mark_as_deleted(notification: &mut Notification) {
        notification.deleted = true;
    }

    pub fn new(
        user_uuid: String,
        id: String,
        dsn: String,
        created_at: String,
        datapoint_created_at: String,
        read: bool,
        deleted: bool,
        notification_type: i32,
        notification_subtype: i32,
    ) -> Self {
        Self {
            user_uuid,
            id,
            dsn,
            created_at,
            datapoint_created_at,
            read,
            deleted,
            notification_type,
            notification_subtype,
        }
    }

    pub fn user_uuid(&self) -> &str {
        &self.user_uuid
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn dsn(&self) -> &str {
        &self.dsn
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn datapoint_created_at(&self) -> &str {
        &self.datapoint_created_at
    }
    pub fn read(&self) -> bool {
        self.read
    }
    pub fn deleted(&self) -> bool {
        self.deleted
    }
    pub fn notification_type(&self) -> i32 {
        self.notification_type
    }
    pub fn notification_subtype(&self) -> i32 {
        self.notification_subtype
    }
}

impl NotificationSubscription {
    pub fn new(
        notification_type: i32,
        notification_subtype: i32,
        subscribed: bool,
    ) -> Self {
        Self {
            notification_type,
            notification_subtype,
            subscribed,
        }
    }
    pub fn notification_type(&self) -> i32 {
        self.notification_type
    }
    pub fn notification_subtype(&self) -> i32 {
        self.notification_subtype
    }
    pub fn subscribed(&self) -> bool {
        self.subscribed
    }
}