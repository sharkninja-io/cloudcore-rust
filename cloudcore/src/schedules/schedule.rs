use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schedule {
    active: Option<bool>,
    day_occur_of_month: Option<Vec<u32>>,
    days_of_month: Option<Vec<u32>>,
    days_of_week: Option<Vec<u32>>,
    device_id: Option<u32>,
    direction: String,
    display_name: Option<String>,
    duration: Option<u32>,
    end_date: Option<String>,
    end_time_each_day: Option<String>,
    fixed_actions: Option<bool>,
    interval: Option<u32>,
    months_of_year: Option<Vec<u32>>,
    name: String,
    start_date: String,
    start_time_each_day: String,
    key: Option<u32>,
    time_before_end: Option<String>,
    utc: Option<bool>,
    version: Option<String>
}

impl Schedule {
    pub fn new(
        active: Option<bool>,
        day_occur_of_month: Option<Vec<u32>>,
        days_of_month: Option<Vec<u32>>,
        days_of_week: Option<Vec<u32>>,
        device_id: Option<u32>,
        direction: String,
        display_name: Option<String>,
        duration: Option<u32>,
        end_date: Option<String>,
        end_time_each_day: Option<String>,
        fixed_actions: Option<bool>,
        interval: Option<u32>,
        months_of_year: Option<Vec<u32>>,
        name: String,
        start_date: String,
        start_time_each_day: String,
        key: Option<u32>,
        time_before_end: Option<String>,
        utc: Option<bool>,
        version: Option<String>
    ) -> Self {
        Self {
            active,
            day_occur_of_month,
            days_of_month,
            days_of_week,
            device_id,
            direction,
            display_name,
            duration,
            end_date,
            end_time_each_day,
            fixed_actions,
            interval,
            months_of_year,
            name,
            start_date,
            start_time_each_day,
            key,
            time_before_end,
            utc,
            version
        }
    }

    pub fn reset(&mut self) {
        self.active = Some(false);
        self.start_time_each_day = "00:00:00".to_string();
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn active(&self) -> Option<bool> {
        self.active
    }
    pub fn day_occur_of_month(&self) -> Option<&Vec<u32>> {
        self.day_occur_of_month.as_ref()
    }
    pub fn days_of_month(&self) -> Option<&Vec<u32>> {
        self.days_of_month.as_ref()
    }
    pub fn days_of_week(&self) -> Option<&Vec<u32>> {
        self.days_of_week.as_ref()
    }
    pub fn device_id(&self) -> Option<u32> {
        self.device_id
    }
    pub fn direction(&self) -> &str {
        &self.direction
    }
    pub fn display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }
    pub fn duration(&self) -> Option<u32> {
        self.duration
    }
    pub fn end_date(&self) -> Option<&String> {
        self.end_date.as_ref()
    }
    pub fn end_time_each_day(&self) -> Option<&String> {
        self.end_time_each_day.as_ref()
    }
    pub fn fixed_actions(&self) -> Option<bool> {
        self.fixed_actions
    }
    pub fn interval(&self) -> Option<u32> {
        self.interval
    }
    pub fn months_of_year(&self) -> Option<&Vec<u32>> {
        self.months_of_year.as_ref()
    }
    pub fn start_date(&self) -> &str {
        &self.start_date
    }
    pub fn start_time_each_day(&self) -> &str {
        &self.start_time_each_day
    }
    pub fn key(&self) -> Option<u32> {
        self.key
    }
    pub fn time_before_end(&self) -> Option<&String> {
        self.time_before_end.as_ref()
    }
    pub fn utc(&self) -> Option<bool> {
        self.utc
    }
    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = Some(active);
    }
    pub fn set_start_time_each_day(&mut self, start_time_each_day: String) {
        self.start_time_each_day = start_time_each_day;
    }
}