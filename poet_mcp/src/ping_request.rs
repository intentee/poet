use serde::Deserialize;
use serde::Serialize;

use crate::ping_request_params::PingRequestParams;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PingRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    #[serde(default)]
    pub params: PingRequestParams,
}
