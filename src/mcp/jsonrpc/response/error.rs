use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::id::Id;

const ERROR_INTERNAL_ERROR: i32 = -32603;
const ERROR_INVALID_PARAMS: i32 = -32602;
const ERROR_INVALID_REQUEST: i32 = -32600;
const ERROR_PARSE_ERROR: i32 = -32700;
const ERROR_RESOURCE_NOT_FOUND: i32 = -32002;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceNotFound {
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolNotFound {
    tool_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Error {
    GenericMessage {
        code: i32,
        jsonrpc: String,
        message: String,
    },
    RequestInternal {
        code: i32,
        id: Id,
        jsonrpc: String,
        message: String,
    },
    ResourceNotFound {
        code: i32,
        data: ResourceNotFound,
        id: Id,
        jsonrpc: String,
        message: String,
    },
    ToolNotFound {
        code: i32,
        data: ToolNotFound,
        id: Id,
        jsonrpc: String,
        message: String,
    },
}

impl Error {
    pub fn invalid_prompt_name(id: Id, name: String) -> Self {
        Self::RequestInternal {
            code: ERROR_INVALID_PARAMS,
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message: format!("Invalid prompt name: {name}"),
        }
    }

    pub fn invalid_request(message: String) -> Self {
        Self::GenericMessage {
            code: ERROR_INVALID_REQUEST,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message,
        }
    }

    pub fn parse(message: String) -> Self {
        Self::GenericMessage {
            code: ERROR_PARSE_ERROR,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message,
        }
    }

    pub fn request_internal(id: Id, message: String) -> Self {
        Self::RequestInternal {
            code: ERROR_INTERNAL_ERROR,
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message,
        }
    }

    pub fn resource_not_found(id: Id, uri: String) -> Self {
        Self::ResourceNotFound {
            code: ERROR_RESOURCE_NOT_FOUND,
            data: ResourceNotFound { uri },
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message: "Resource not found".to_string(),
        }
    }

    pub fn tool_not_found(id: Id, tool_name: String) -> Self {
        Self::ToolNotFound {
            code: ERROR_RESOURCE_NOT_FOUND,
            data: ToolNotFound { tool_name },
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            message: "Tool not found".to_string(),
        }
    }
}
