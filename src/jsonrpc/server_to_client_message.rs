use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::response::success::Success;
use crate::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::jsonrpc::response::success::pong::Pong;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerToClientMessage {
    InitializeResult(Success<InitializeResult>),
    Pong(Success<Pong>),
}
