use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

use crate::json_rpc_error_code::JsonRpcErrorCode;
use crate::mcp_error::McpError;

const fn error_code(mcp_error: &McpError) -> JsonRpcErrorCode {
    match mcp_error {
        McpError::ParseMessage { .. } => JsonRpcErrorCode::ParseError,
        McpError::InvalidAcceptHeader { .. }
        | McpError::InvalidSessionHeader { .. }
        | McpError::MissingProtocolVersionHeader
        | McpError::MissingSessionHeader
        | McpError::NotAcceptable
        | McpError::ReadPayload { .. }
        | McpError::SessionNotFound { .. }
        | McpError::UnexpectedSessionHeader
        | McpError::UnsupportedProtocolVersion { .. } => JsonRpcErrorCode::InvalidRequest,
        McpError::AlreadySubscribed { .. }
        | McpError::DeserializeToolInput { .. }
        | McpError::EmptyListPage
        | McpError::InvalidResourceUri { .. }
        | McpError::MissingResourceUriAuthority { .. }
        | McpError::MissingResourceUriScheme { .. }
        | McpError::NoResourceProvider { .. }
        | McpError::PromptNotFound { .. }
        | McpError::ToolNotFound { .. } => JsonRpcErrorCode::InvalidParams,
        McpError::ResourceNotFound { .. } => JsonRpcErrorCode::ResourceNotFound,
        McpError::PromptProviderFailed { .. }
        | McpError::ResourceProviderFailed { .. }
        | McpError::SerializeToolOutput { .. }
        | McpError::ToolFailed { .. } => JsonRpcErrorCode::InternalError,
    }
}

fn error_data(mcp_error: &McpError) -> Option<Value> {
    match mcp_error {
        McpError::ResourceNotFound { uri } => Some(json!({ "uri": uri })),
        McpError::ToolNotFound { tool_name } => Some(json!({ "tool_name": tool_name })),
        _ => None,
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorObject {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    pub message: String,
}

impl From<&McpError> for JsonRpcErrorObject {
    fn from(mcp_error: &McpError) -> Self {
        Self {
            code: error_code(mcp_error) as i32,
            data: error_data(mcp_error),
            message: mcp_error.to_string(),
        }
    }
}
