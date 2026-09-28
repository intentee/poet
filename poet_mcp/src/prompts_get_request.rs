use serde::Deserialize;
use serde::Serialize;

use crate::prompts_get_request_params::PromptsGetRequestParams;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptsGetRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: PromptsGetRequestParams,
}
