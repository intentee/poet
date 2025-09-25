use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::notification::Notification;
use crate::jsonrpc::notification::initialized::Initialized;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;
use crate::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::jsonrpc::request::ping::Ping;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ClientToServerMessage {
    Initialize(Request<Initialize>),
    Initialized(Notification<Initialized>),
    LoggingSetLevel(Request<LoggingSetLevel>),
    Ping(Request<Ping>),
}
