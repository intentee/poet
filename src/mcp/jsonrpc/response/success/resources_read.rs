use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::meta::Meta;
use crate::mcp::resource_content::ResourceContent;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesRead {
    pub contents: Vec<ResourceContent>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}
