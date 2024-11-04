use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeZone {
    pub utc_offset: Option<String>,
    pub dst: bool,
    pub dst_active: bool,
    pub dst_next_change_time: Option<String>,
    pub tz_id: Option<String>,
}