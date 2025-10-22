use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::mcp::jsonrpc::response::success::prompts_get_result::PromptsGetResult;
use crate::mcp::jsonrpc::response::success::prompts_list::PromptsList;
use crate::mcp::jsonrpc::response::success::resource_templates_list::ResourcesTemplatesList;
use crate::mcp::jsonrpc::response::success::resources_list::ResourcesList;
use crate::mcp::jsonrpc::response::success::resources_read::ResourcesRead;
use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::jsonrpc::response::success::tools_list::ToolsList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerToClientResponse {
    EmptyResponse(Success<EmptyResponse>),
    Error(Error),
    InitializeResult(Success<InitializeResult>),
    PromptsGet(Success<PromptsGetResult>),
    PromptsList(Success<PromptsList>),
    ResourcesList(Success<ResourcesList>),
    ResourcesRead(Success<ResourcesRead>),
    ResourcesTemplatesList(Success<ResourcesTemplatesList>),
    ToolsCall(Success<ToolCallResult<Value>>),
    ToolsList(Success<ToolsList>),
}
