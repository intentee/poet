use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;
use crate::jsonrpc::request::ping::Ping;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ClientToServerMessage {
    Initialize(Request<Initialize>),
    Ping(Request<Ping>),
}
