use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::params_with_meta::ParamsWithMeta;
use crate::mcp::log_level::LogLevel;

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
