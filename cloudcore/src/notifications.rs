#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use std::time::SystemTime;

#[cfg(feature = "library")]
use chrono::{DateTime, Utc};
#[cfg(feature = "library")]
use chrono::Duration;
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use uuid::Uuid;

#[cfg(feature = "library")]
use crate::cache::{CacheDataValue, CacheInteract};
#[cfg(feature = "library")]
use crate::cache::CacheDir;
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "library")]
use crate::notifications::notifications::Notification;
#[cfg(feature = "library")]
use crate::properties::datapoint::IoTDatapoint;
#[cfg(feature = "library")]
use crate::properties::trigger::ERROR_NOTIFICATION_PROPERTY_NAME;
#[cfg(feature = "library")]
use crate::properties::trigger::NOTIFICATION_TYPE_ERROR;

pub mod notifications;


#[cfg(feature = "library")]
impl CloudCore {
    async fn cache_notifications(&mut self, dsn: String, notifications: Vec<Notification>) {
        if !self.cache.child_paths().contains_key(&dsn) {
            if self.cache.make_dir_for_child(&dsn).is_ok() {
                debug!("cache made for notifications");
            } else {
                error!("failed to make cache for notifications");
            }
        }
        debug!("Caching {} notifications", notifications.len());
        if let Some(str) = serde_json::to_string(&notifications).ok() {
            debug!("Serializing notifications str: {}", str);
            let _ = self.cache.set_value(dsn, "notifications".to_string(), str);
        }
    }

    pub async fn delete_all_notifications(
        &mut self,
        to: String) -> Result<(), Box<dyn Error>> {
        let devices = self.fetch_all_devices().await?;
        let mut results = true;
        for device in devices {
            let opt_dsn = device.dsn();
            match opt_dsn {
                None => {
                    debug!("Invalid empty DSN")
                }
                Some(dsn) => {
                    results = results && self.mark_notifications_as_deleted(dsn.to_string(), to.to_string()).await;
                }
            }
        }
        Ok(())
    }

    pub async fn delete_notification(
        &mut self,
        dsn: String,
        id: String) -> Result<(), Box<dyn Error>> {
        self.mark_notification_as_deleted(dsn.to_string(), id.to_string()).await;
        Ok(())
    }

    pub async fn fetch_all_notifications(
        &mut self,
        from: String) -> Result<Vec<Notification>, Box<dyn Error>> {
        let devices = self.fetch_all_devices().await?;
        let mut results = vec![];
        for device in devices {
            let opt_dsn = device.dsn;
            match opt_dsn {
                None => {}
                Some(dsn) => {
                    let device_notifications = self.fetch_notifications(dsn.to_string(), from.to_string()).await?;
                    for notification in device_notifications {
                        results.push(notification);
                    }
                }
            }
        }
        Ok(results)
    }

    pub async fn fetch_notifications(
        &mut self,
        dsn: String,
        from: String,
    ) -> Result<Vec<Notification>, Box<dyn Error>> {
        let oldest_age = Utc::now() - Duration::days(Notification::MAX_AGE_DAYS);
        let qry_from = if from.is_empty() {
            oldest_age.to_rfc3339()
        } else {
            from
        };
        let error_datapoints = self.get_datapoints(
            dsn.to_string(),
            ERROR_NOTIFICATION_PROPERTY_NAME.to_string(),
            None,
            Some(qry_from),
            None,
            "".to_string(),
        ).await.0?;

        if self.user_session.is_none() {
            return Err("no user session".into());
        }
        let user_uuid = self.user_session.as_ref().unwrap().user_uuid().unwrap().to_string();
        let mut fetched_notifications = vec![];

        let cached_notifications = self.get_cached_notifications(dsn.to_string()).await;

        let dt: DateTime<Utc> = SystemTime::now().into();
        let created_at = format!("{}", dt.format("%+"));
        error_datapoints.into_iter().for_each(|error_datapoint: IoTDatapoint| {
            let alert_id = Uuid::new_v4();
            let subtype = error_datapoint.value().int_value().unwrap().clone();
            if subtype != 0 {
                fetched_notifications.push(
                    Notification::new(
                        user_uuid.to_string(),
                        alert_id.to_string(),
                        dsn.to_string(),
                        created_at.to_string(),
                        error_datapoint.created_at().unwrap().to_string(),
                        false,
                        false,
                        NOTIFICATION_TYPE_ERROR,
                        subtype,
                    )
                );
            }
        });

        let mut notifications: Vec<Notification> = vec![];

        if cached_notifications.is_none() || cached_notifications.as_ref().unwrap().len() == 0 {
            // No cached notifications -- cache everything
            debug!("No cached notifications. Returning all fetched...");
            fetched_notifications.iter().for_each(|fetched_notification: &Notification| {
                notifications.push(fetched_notification.clone());
            });
        } else {
            // Cached notifications exist -- find matches and cache new
            debug!("Cached notifications exist. Merging....");
            fetched_notifications.iter().for_each(|fetched_notification: &Notification| {
                let mut matched_notification: Option<Notification> = None;
                // Find if notification is cached
                cached_notifications.as_ref().unwrap().iter().for_each(|cached_notification| {
                    if Notification::matches(cached_notification, fetched_notification) {
                        matched_notification = Some(cached_notification.clone());
                    }
                });

                let notification = if matched_notification.is_some() {
                    // Return the cached version
                    matched_notification.unwrap()
                } else {
                    fetched_notification.clone()
                };
                notifications.push(notification);
            });
        }

        self.cache_notifications(dsn.to_string(), notifications.clone()).await;
        Ok(notifications.into_iter()
            .filter(|n| !n.deleted)
            .collect())
    }


    pub async fn get_all_cached_notifications(
        &mut self) -> Result<Vec<Notification>, Box<dyn Error>> {
        let devices = self.fetch_all_devices().await?;
        let mut results = vec![];
        for device in devices {
            match device.dsn() {
                None => { debug!("Invalid empty DSN"); }
                Some(dsn) => {
                    let device_notifications = self.get_cached_notifications(dsn.to_string()).await;
                    match device_notifications {
                        None => { debug!("Invalid empty notifications"); }
                        Some(device_notifications) => {
                            let mut count = 0;
                            for notification in device_notifications {
                                results.push(notification);
                                count += 1;
                            }
                            debug!("Found {} cached notifications for device", count)
                        }
                    }
                }
            }
        }
        Ok(results)
    }

    pub async fn get_cached_notifications(&self, dsn: String) -> Option<Vec<Notification>> {
        let cache_data = self.cache.get_value(dsn, "notifications".to_string());
        if let Ok(data) = cache_data {
            match data {
                CacheDataValue::StringValue(val) => {
                    let notifications: Option<Vec<Notification>> = serde_json::from_str(&*val).unwrap_or_else(|err| {
                        error!("Error getting cached notifications: {}", err);
                        panic!("Error getting cached notifications: {}", err)
                    });
                    notifications
                }
                CacheDataValue::NullValue => None,
                _ => {
                    error!("Notifications not saved as CacheDataValue::StringValue");
                    panic!("Notifications not saved as CacheDataValue::StringValue")
                }
            }
        } else {
            debug!("No cached notifications: {}", cache_data.err().unwrap().to_string());
            None
        }
    }

    pub async fn mark_all_notifications_as_read(
        &mut self) -> Result<(), Box<dyn Error>> {
        let devices = self.fetch_all_devices().await?;
        let mut results = true;
        for device in devices {
            match device.dsn() {
                None => { debug!("Invalid empty DSN"); }
                Some(dsn) => {
                    results = results && self.mark_notifications_as_read(dsn.to_string()).await;
                }
            }
        }
        Ok(())
    }

    pub async fn mark_notifications_as_read(&mut self, dsn: String) -> bool {
        debug!("Marking all Notifications for Device({}) as READ", dsn);
        self.update_all(dsn, Notification::mark_as_read).await
    }

    pub async fn mark_notifications_as_deleted(&mut self, dsn: String, to: String) -> bool {
        debug!("Marking Notifications to {} for Device({}) as DELETED", to, dsn);
        self.update_to(dsn, to, Notification::mark_as_deleted).await
    }

    pub async fn mark_notification_as_read(&mut self, dsn: String, id: String) -> bool {
        debug!("Marking Notification({}) for Device({}) as READ", dsn, id);
        self.update(dsn, id, Notification::mark_as_read).await
    }

    pub async fn mark_notification_as_deleted(&mut self, dsn: String, id: String) -> bool {
        debug!("Marking Notification({}) for Device({}) as DELETED", dsn, id);
        self.update(dsn, id, Notification::mark_as_deleted).await
    }

    async fn update_all(&mut self, dsn: String, update_fn: fn(&mut Notification)) -> bool {
        let cached_notifications = self.get_cached_notifications(dsn.to_string()).await;
        match cached_notifications {
            None => { false }
            Some(mut notifications) => {
                notifications.iter_mut().for_each(|n| update_fn(n));
                self.cache_notifications(dsn.to_string(), notifications).await;
                true
            }
        }
    }

    async fn update_to(&mut self, dsn: String, to: String, update_fn: fn(&mut Notification)) -> bool {
        let cached_notifications = self.get_cached_notifications(dsn.to_string()).await;
        match cached_notifications {
            None => {
                debug!("update_to: none to update for DSN {} up to {}", dsn.to_string(), to.to_string());
                false
            }
            Some(mut notifications) => {
                debug!("update_to: {} to update for DSN {} up to {}",  notifications.len(), dsn.to_string(), to.to_string());
                notifications.iter_mut().filter(|n| {
                    let created_millis = DateTime::parse_from_rfc3339(n.datapoint_created_at.as_str()).unwrap().timestamp_millis();
                    let to_millis = DateTime::parse_from_rfc3339(to.as_str()).unwrap().timestamp_millis();
                    let res = created_millis < to_millis;
                    debug!("datapoint_created_at {} < to {}", n.datapoint_created_at.as_str(), to.as_str());
                    debug!("created_millis {} < to_millis {}: {}", created_millis, to_millis, res);
                    return res;
                }).for_each(|n| update_fn(n));
                self.cache_notifications(dsn.to_string(), notifications).await;
                true
            }
        }
    }

    async fn update(&mut self, dsn: String, id: String, update_fn: fn(&mut Notification)) -> bool {
        let cached_notifications = self.get_cached_notifications(dsn.to_string()).await;
        match cached_notifications {
            None => { false }
            Some(mut notifications) => {
                notifications.iter_mut().for_each(|n| if n.id == id { update_fn(n) });
                self.cache_notifications(dsn.to_string(), notifications).await;
                true
            }
        }
    }

}