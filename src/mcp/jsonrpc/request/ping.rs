use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "ping", tag = "method")]
pub struct Ping {
    pub id: Id,
    pub jsonrpc: String,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub method: String,
    pub params: EmptyObject,
}
