use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub const ERROR_CODE_PARSE_ERROR: i32 = -32700;
pub const ERROR_INVALID_REQUEST: i32 = -32600;
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
pub const ERROR_INVALID_PARAMS: i32 = -32602;
pub const ERROR_INTERNAL_ERROR: i32 = -32603;
pub const ERROR_SERVER_ERROR_RANGE_MIN: i32 = -32099;
pub const ERROR_SERVER_ERROR_RANGE_MAX: i32 = -32000;

#[derive(Deserialize, Serialize)]
pub struct Error {
    pub code: i32,
    pub data: Value,
    pub id: String,
    pub jsonrpc: String,
}
