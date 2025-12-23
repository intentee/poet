use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::meta::Meta;
use crate::mcp::list_resources_cursor::ListResourcesCursor;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsListParams {
    #[serde(default, with = "crate::mcp::list_resources_cursor")]
    pub cursor: Option<ListResourcesCursor>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsList {
    pub id: Id,
    pub jsonrpc: String,
    pub params: ToolsListParams,
}
