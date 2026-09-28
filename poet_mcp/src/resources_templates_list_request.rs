use serde::Deserialize;
use serde::Serialize;

use crate::request_id::RequestId;
use crate::resources_templates_list_request_params::ResourcesTemplatesListRequestParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesTemplatesListRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    #[serde(default)]
    pub params: ResourcesTemplatesListRequestParams,
}
