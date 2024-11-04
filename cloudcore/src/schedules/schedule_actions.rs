use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleAction {
    name: String,
    base_type: String,
    in_range: Option<bool>,
    at_start: Option<bool>,
    at_end: Option<bool>,
    active: Option<bool>,
    key: Option<String>,
    value: Option<String>
}

impl ScheduleAction {
    pub fn new(
        name: String,
        base_type: String,
        in_range: Option<bool>,
        at_start: Option<bool>,
        at_end: Option<bool>,
        active: Option<bool>,
        key: Option<String>,
        value: Option<String>
    ) -> Self {
        Self {
            name,
            base_type,
            in_range,
            at_start,
            at_end,
            active,
            key,
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn base_type(&self) -> &str {
        &self.base_type
    }
    pub fn in_range(&self) -> Option<bool> {
        self.in_range
    }
    pub fn at_start(&self) -> Option<bool> {
        self.at_start
    }
    pub fn at_end(&self) -> Option<bool> {
        self.at_end
    }
    pub fn active(&self) -> Option<bool> {
        self.active
    }
    pub fn key(&self) -> Option<&String>  {
        self.key.as_ref()
    }
    pub fn value(&self) -> Option<&String>  {
        self.value.as_ref()
    }
}