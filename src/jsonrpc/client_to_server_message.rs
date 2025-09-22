use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::notification::Notification;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ClientToServerMessage {
    Initialize(Request<Initialize>),
    Notification(Notification),
}
