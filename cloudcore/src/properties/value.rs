use serde::{Deserialize, Serialize};

/// If there's no indication (besides structure) of result's type,
///  you can use the untagged enum representation. This will try
/// deserializing to each variant in turn until a matching one is found:
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum IoTPropertyValue {
    Int(i32),
    Str(String),
    Bool(bool),
}

impl IoTPropertyValue {
    pub fn bool_value(&self) -> Option<&bool> {
        match &self {
            Self::Bool(it) => Some(it),
            Self::Int(it) => {
                let owned = it.to_owned();
                let falsy: i32 = 0;
                let truthy: i32 = 1;
                return if owned == falsy {
                    Some(&false)
                } else if owned == truthy {
                    Some(&true)
                } else {
                    Option::None
                };
            }
            _ => Option::None,
        }
    }

    pub fn int_value(&self) -> Option<&i32> {
        match &self {
            Self::Int(it) => Some(it),
            Self::Bool(it) => {
                let owned = it.to_owned();
                let falsy: bool = false;
                let truthy: bool = true;
                return if owned == falsy {
                    Some(&0)
                } else if owned == truthy {
                    Some(&1)
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        }
    }

    pub fn string_value(&self) -> Option<&String> {
        match &self {
            Self::Str(it) => Some(it),
            _ => Option::None,
        }
    }
}