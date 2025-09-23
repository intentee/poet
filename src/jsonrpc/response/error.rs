use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::jsonrpc::JSONRPC_VERSION;
use crate::jsonrpc::id::Id;

// pub const ERROR_PARSE_ERROR: i32 = -32700;
pub const ERROR_INVALID_REQUEST: i32 = -32600;
// pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
// pub const ERROR_INVALID_PARAMS: i32 = -32602;
// pub const ERROR_INTERNAL_ERROR: i32 = -32603;
// pub const ERROR_SERVER_ERROR_RANGE_MIN: i32 = -32099;
// pub const ERROR_SERVER_ERROR_RANGE_MAX: i32 = -32000;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: i32,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    pub jsonrpc: String,
}

impl Error {
    pub fn invalid_request(message: String) -> Self {
        Self {
            code: ERROR_INVALID_REQUEST,
            data: format!("Invalid request: {message}").into(),
            id: None,
            jsonrpc: JSONRPC_VERSION.to_string(),
        }
    }
}
