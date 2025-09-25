use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::mcp::jsonrpc::response::success::resources_list::ResourcesList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerToClientMessage {
    EmptyResponse(Success<EmptyResponse>),
    InitializeResult(Success<InitializeResult>),
    ResourcesList(Success<ResourcesList>),
}
