use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::params_with_meta::ParamsWithMeta;

#[derive(Debug, Deserialize, Serialize)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emergency")]
    Emergency,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingSetLevelParams {
    pub level: LogLevel,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "logging/setLevel", tag = "method")]
pub struct LoggingSetLevel {
    pub method: String,
    pub params: ParamsWithMeta<LoggingSetLevelParams>,
}
