use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::resources_unsubscribe_request_params::ResourcesUnsubscribeRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesUnsubscribeRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: ResourcesUnsubscribeRequestParams,
}
