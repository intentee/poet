use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::params_with_meta::ParamsWithMeta;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourcesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "resources/list", tag = "method")]
pub struct ResourcesList {
    pub method: String,
    pub params: ParamsWithMeta<ResourcesListParams>,
}
