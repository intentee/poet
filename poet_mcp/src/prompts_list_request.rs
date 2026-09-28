use serde::Deserialize;
use serde::Serialize;

use crate::prompts_list_request_params::PromptsListRequestParams;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptsListRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    #[serde(default)]
    pub params: PromptsListRequestParams,
}
