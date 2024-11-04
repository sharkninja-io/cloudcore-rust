mod schedule;
pub use self::schedule::{Schedule};
mod schedule_actions;
pub use self::schedule_actions::{ScheduleAction};

#[cfg(feature = "library")]
use crate::cloudcore::CloudCore;
#[cfg(feature = "library")]
use log::debug;
#[cfg(feature = "library")]
use serde::Deserialize;
#[cfg(feature = "library")]
use serde::Serialize;
#[cfg(feature = "library")]
use std::error::Error;
#[cfg(feature = "library")]
use log::error;
#[cfg(feature = "library")]
use crate::properties::property::{PROPS_PATH_PARAMS_DSN, PROPS_PATH_PARAMS_PROP_NAME};
#[cfg(feature = "library")]
use crate::urls;
#[cfg(feature = "library")]
use crate::ErrorUtil;

#[cfg(feature = "library")]
#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    schedule: Schedule,
}

#[cfg(feature = "library")]
impl CloudCore {
    pub async fn create_device_schedule(
        &mut self, 
        dsn: String, 
        name: String, 
        start_date: String, 
        start_time_each_day: String,
        action_name: String,
        action_base_type: String
    ) -> Result<Schedule, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let mut url = String::from(&self.session_params().device_url);
        let endpoint = String::from(urls::AYLA_DEVICE_SCHEDULE_JSON).replace(PROPS_PATH_PARAMS_DSN, &dsn);
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();

        let auth_bearer = format!("auth_token {}", token);

        let client = self.client();

        // local structs for schedules request
        #[derive(Serialize, Debug)]
        struct SchedulePayload {
            schedule: Schedule,
            schedule_actions: ScheduleAction,
        }

        let schedule = Schedule::new(
            Some(true),
            None,
            None,
            None,
            None,
            "input".to_string(),
            None,
            None,
            None,
            None,
            Some(true),
            None,
            None,
            name,
            start_date,
            start_time_each_day,
            None,
            None,
            Some(true),
            None
        );

        let schedule_actions = ScheduleAction::new(
            action_name,
            action_base_type,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let post_data = SchedulePayload {
            schedule,
            schedule_actions
        };

        debug!("create device schedules request data: {:#?}", post_data);

        let response = client
            .post(url)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .json(&post_data)
            .send()
            .await?;

        if response.status().is_success() {
            let create_account_payload = response.json::<ScheduleResponse>().await?;
            debug!("create device schedules payload: {:#?}", create_account_payload);
            Ok(create_account_payload.schedule)
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn fetch_schedules(
        &self,
        device_id: Option<u32>,
    ) -> Result<Vec<Schedule>, Box<dyn Error>> {
        match device_id {
            Some(val) => {
                let schedules = self.fetch_device_schedules(val).await?;
                Ok(schedules)
            },
            None => {
                let schedules = self.fetch_all_schedules().await?;
                Ok(schedules)
            }
        }
    }

    pub async fn fetch_device_schedules(
        &self,
        device_id: u32
    ) -> Result<Vec<Schedule>, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let mut url = String::from(&self.session_params().user_url);
        let endpoint = String::from(urls::AYLA_DEVICE_ID_SCHEDULE_JSON)
                    .replace(PROPS_PATH_PARAMS_DSN, &device_id.to_string());
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();

        let auth_bearer = format!("auth_token {}", token);

        let client = self.client();

        let response = client
            .get(url)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        let schedules_payload = response.json::<Vec<ScheduleResponse>>().await?;
        let mut schedules: Vec<Schedule> = vec![];
        schedules_payload.into_iter().for_each(|response| {
            debug!("Device Schedule: {:?}", &response.schedule);
            schedules.push(response.schedule);
        });
        Ok(schedules)
    }

    pub async fn fetch_all_schedules(
        &self
    ) -> Result<Vec<Schedule>, Box<dyn Error>> {
        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let mut url = String::from(&self.session_params().device_url);
        url.push_str(urls::AYLA_DEVICE_USER_SCHEDULES_JSON);

        let token = self.user_session.as_ref().unwrap().access_token();

        let auth_bearer = format!("auth_token {}", token);

        let client = self.client();

        let response = client
            .get(url)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .send()
            .await?;
        let schedules_payload = response.json::<Vec<ScheduleResponse>>().await?;
        let mut schedules: Vec<Schedule> = vec![];
        schedules_payload.into_iter().for_each(|response| {
            debug!("User Schedule: {:?}", &response.schedule);
            schedules.push(response.schedule);
        });
        Ok(schedules)
    }

    pub async fn update_schedule(
        &self,
        schedule: Schedule,
    ) -> Result<Schedule, Box<dyn Error>> {

        if self.user_session.is_none() {
            return Err(Box::new(ErrorUtil::user_session_not_found_error()));
        }

        let mut url = String::from(&self.session_params().device_url);
        let endpoint = String::from(urls::AYLA_DEVICE_UPDATE_SCHEDULE_JSON)
                                .replace(PROPS_PATH_PARAMS_DSN, &schedule.device_id().unwrap().to_string())
                                .replace(PROPS_PATH_PARAMS_PROP_NAME, &schedule.key().unwrap().to_string());
        url.push_str(&endpoint);

        let token = self.user_session.as_ref().unwrap().access_token();

        let auth_bearer = format!("auth_token {}", token);

        let client = self.client();

        // local structs for schedules request
        #[derive(Serialize, Debug)]
        struct ScheduleObjectPayload {
            active: bool,
            start_time_each_day: String,
            direction: String,
            utc: String
        }

        #[derive(Serialize, Debug)]
        struct ScheduleActionPayload {
            active: bool,
        }

        #[derive(Serialize, Debug)]
        struct SchedulePayload {
            schedule: Schedule,
        }

        let schedule = schedule.clone();

        let post_data = SchedulePayload {
            schedule,
        };

        debug!("update device schedules request data: {:#?}", post_data);

        let response = client
            .put(url)
            .header(urls::AUTHORIZATION_HEADER, auth_bearer)
            .json(&post_data)
            .send()
            .await?;

        if response.status().is_success() {
            let update_account_payload = response.json::<ScheduleResponse>().await?;
            debug!("update device schedules payload: {:#?}", update_account_payload);
            Ok(update_account_payload.schedule)
        } else {
            let error_payload = response.text().await?;
            Err(Box::new(ErrorUtil::server_error(error_payload)))
        }
    }

    pub async fn clear_schedules(&self, key: u32) -> Result<(), Box<dyn Error>> {
        let schedules = self.fetch_schedules(Some(key)).await?;
        // TODO: To do concurrently need to have a callback passed
        for mut sched in schedules {
            sched.reset();
            if let Some(err) = self.update_schedule(sched).await.err() {
                error!("Error clearing schedule: {}", err.to_string());
            }
        }
        Ok(())
    }
}