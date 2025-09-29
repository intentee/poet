use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::id::Id;

// pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
// pub const ERROR_INVALID_PARAMS: i32 = -32602;
// pub const ERROR_INTERNAL_ERROR: i32 = -32603;
// pub const ERROR_SERVER_ERROR_RANGE_MIN: i32 = -32099;
// pub const ERROR_SERVER_ERROR_RANGE_MAX: i32 = -32000;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceNotFound {
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    InvalidRequest = -32600,
    ParseError = -32700,
    ResourceNotFound = -32002,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Error {
    GenericMessage {
        code: ErrorCode,
        jsonrpc: String,
        message: String,
    },
    ResourceNotFound {
        code: ErrorCode,
        data: ResourceNotFound,
        id: Id,
        jsonrpc: String,
        message: String,
    },
}

impl Error {
    pub fn invalid_request(message: String) -> Self {
        Self::GenericMessage {
            code: ErrorCode::InvalidRequest,
            message,
            jsonrpc: JSONRPC_VERSION.to_string(),
        }
    }

    pub fn parse(message: String) -> Self {
        Self::GenericMessage {
            code: ErrorCode::ParseError,
            message,
            jsonrpc: JSONRPC_VERSION.to_string(),
        }
    }

    pub fn resource_not_found(id: Id, uri: String) -> Self {
        Self::ResourceNotFound {
            code: ErrorCode::ResourceNotFound,
            data: ResourceNotFound { uri },
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message: "Resource not found".to_string(),
        }
    }
}
