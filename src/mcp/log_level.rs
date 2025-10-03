use std::cmp::Ordering;

use serde::Deserialize;
use serde::Serialize;

#[repr(u8)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug = 0,
    #[serde(rename = "info")]
    Info = 1,
    #[serde(rename = "notice")]
    Notice = 2,
    #[serde(rename = "warning")]
    Warning = 3,
    #[serde(rename = "error")]
    Error = 4,
    #[serde(rename = "critical")]
    Critical = 5,
    #[serde(rename = "alert")]
    Alert = 6,
    #[serde(rename = "emergency")]
    Emergency = 7,
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.clone() as u8).cmp(&(other.clone() as u8))
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
