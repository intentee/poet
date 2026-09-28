use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::tools_list_request_params::ToolsListRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsListRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    #[serde(default)]
    pub params: ToolsListRequestParams,
}
