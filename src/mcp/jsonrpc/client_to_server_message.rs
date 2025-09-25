use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::initialized::Initialized;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::ping::Ping;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ClientToServerMessage {
    #[serde(rename = "initialize")]
    Initialize(Initialize),
    #[serde(rename = "notifications/initialized")]
    Initialized(Initialized),
    #[serde(rename = "logging/setLevel")]
    LoggingSetLevel(LoggingSetLevel),
    #[serde(rename = "ping")]
    Ping(Ping),
    #[serde(rename = "resources/list")]
    ResourcesList(ResourcesList),
}
