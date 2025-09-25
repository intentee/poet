use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::initialized::Initialized;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::ping::Ping;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientToServerMessage {
    Initialize(Initialize),
    Initialized(Initialized),
    LoggingSetLevel(LoggingSetLevel),
    Ping(Ping),
    ResourcesList(ResourcesList),
}
