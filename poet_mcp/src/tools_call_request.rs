use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::tools_call_request_params::ToolsCallRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsCallRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: ToolsCallRequestParams,
}
