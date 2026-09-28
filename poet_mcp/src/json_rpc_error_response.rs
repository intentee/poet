use serde::Deserialize;
use serde::Serialize;

use crate::json_rpc_error_object::JsonRpcErrorObject;
use crate::jsonrpc_version::JSONRPC_VERSION;
use crate::mcp_error::McpError;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorResponse {
    pub error: JsonRpcErrorObject,
    pub id: Option<RequestId>,
    pub jsonrpc: String,
}

impl JsonRpcErrorResponse {
    #[must_use]
    pub fn from_mcp_error(id: Option<RequestId>, mcp_error: &McpError) -> Self {
        Self {
            error: JsonRpcErrorObject::from(mcp_error),
            id,
            jsonrpc: JSONRPC_VERSION.to_owned(),
        }
    }
}
