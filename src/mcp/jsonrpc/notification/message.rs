use serde::Deserialize;
use serde::Serialize;

use crate::mcp::log_level::LogLevel;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MessageParams {
    pub data: String,
    pub level: LogLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub jsonrpc: String,
    pub params: MessageParams,
}
