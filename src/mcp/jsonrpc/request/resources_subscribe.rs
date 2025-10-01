use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesSubscribeParams {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesSubscribe {
    pub id: Id,
    pub jsonrpc: String,
    pub params: ResourcesSubscribeParams,
}
