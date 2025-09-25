use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::params_with_meta::ParamsWithMeta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "ping", tag = "method")]
pub struct Ping {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<ParamsWithMeta<EmptyObject>>,
}
