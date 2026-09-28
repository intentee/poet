use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::empty_result::EmptyResult;
use crate::initialize_result::InitializeResult;
use crate::json_rpc_error_response::JsonRpcErrorResponse;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::prompts_get_result::PromptsGetResult;
use crate::prompts_list_result::PromptsListResult;
use crate::resources_list_result::ResourcesListResult;
use crate::resources_read_result::ResourcesReadResult;
use crate::resources_templates_list_result::ResourcesTemplatesListResult;
use crate::tool_call_result::ToolCallResult;
use crate::tools_list_result::ToolsListResult;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerToClientResponse {
    EmptyResult(JsonRpcSuccessResponse<EmptyResult>),
    Error(JsonRpcErrorResponse),
    InitializeResult(JsonRpcSuccessResponse<InitializeResult>),
    PromptsGet(JsonRpcSuccessResponse<PromptsGetResult>),
    PromptsList(JsonRpcSuccessResponse<PromptsListResult>),
    ResourcesList(JsonRpcSuccessResponse<ResourcesListResult>),
    ResourcesRead(JsonRpcSuccessResponse<ResourcesReadResult>),
    ResourcesTemplatesList(JsonRpcSuccessResponse<ResourcesTemplatesListResult>),
    ToolsCall(JsonRpcSuccessResponse<ToolCallResult<Value>>),
    ToolsList(JsonRpcSuccessResponse<ToolsListResult>),
}
