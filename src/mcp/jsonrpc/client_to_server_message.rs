use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::Notification;
use crate::mcp::jsonrpc::notification::initialized::Initialized;
use crate::mcp::jsonrpc::request::Request;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::ping::Ping;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ClientToServerMessage {
    Initialize(Request<Initialize>),
    Initialized(Notification<Initialized>),
    LoggingSetLevel(Request<LoggingSetLevel>),
    Ping(Request<Ping>),
    ResourcesList(Request<ResourcesList>),
}
