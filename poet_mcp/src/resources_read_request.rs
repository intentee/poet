use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::resources_read_request_params::ResourcesReadRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesReadRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: ResourcesReadRequestParams,
}
