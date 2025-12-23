use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::meta::Meta;
use crate::mcp::log_level::LogLevel;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingSetLevelParams {
    pub level: LogLevel,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingSetLevel {
    pub id: Id,
    pub jsonrpc: String,
    pub params: LoggingSetLevelParams,
}
