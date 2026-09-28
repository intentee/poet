use serde::Deserialize;
use serde::Serialize;

use crate::initialize_request_params::InitializeRequestParams;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitializeRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: InitializeRequestParams,
}
