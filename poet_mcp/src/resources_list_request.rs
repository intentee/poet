use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::resources_list_request_params::ResourcesListRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesListRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    #[serde(default)]
    pub params: ResourcesListRequestParams,
}
