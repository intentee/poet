use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::resources_subscribe_request_params::ResourcesSubscribeRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesSubscribeRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: ResourcesSubscribeRequestParams,
}
