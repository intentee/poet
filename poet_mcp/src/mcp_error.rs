use actix_web::HttpResponse;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use actix_web::http::header::ToStrError;

use crate::json_rpc_error_response::JsonRpcErrorResponse;
use crate::provider_error::ProviderError;

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("already subscribed to '{uri}'")]
    AlreadySubscribed { uri: String },
    #[error("invalid input for tool '{tool_name}'")]
    DeserializeToolInput {
        tool_name: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("per_page must be greater than 0")]
    EmptyListPage,
    #[error("unable to parse the Accept header")]
    InvalidAcceptHeader {
        #[source]
        source: actix_web::error::ParseError,
    },
    #[error("invalid resource URI '{uri}'")]
    InvalidResourceUri {
        uri: String,
        #[source]
        source: http::uri::InvalidUri,
    },
    #[error("the Mcp-Session-Id header contains characters other than visible ASCII")]
    InvalidSessionHeader {
        #[source]
        source: ToStrError,
    },
    #[error("the Mcp-Protocol-Version header is required")]
    MissingProtocolVersionHeader,
    #[error("resource URI '{uri}' has no authority")]
    MissingResourceUriAuthority { uri: String },
    #[error("resource URI '{uri}' has no scheme")]
    MissingResourceUriScheme { uri: String },
    #[error("the Mcp-Session-Id header is required")]
    MissingSessionHeader,
    #[error("no resource provider handles '{uri}'")]
    NoResourceProvider { uri: String },
    #[error("the Accept header does not allow the response content types")]
    NotAcceptable,
    #[error("unable to parse the JSON-RPC message")]
    ParseMessage {
        #[source]
        source: serde_json::Error,
    },
    #[error("prompt '{prompt_name}' not found")]
    PromptNotFound { prompt_name: String },
    #[error("prompt provider failed")]
    PromptProviderFailed {
        #[source]
        source: ProviderError,
    },
    #[error("unable to read the request payload")]
    ReadPayload {
        #[source]
        source: actix_web::Error,
    },
    #[error("resource '{uri}' not found")]
    ResourceNotFound { uri: String },
    #[error("resource provider failed for '{uri}'")]
    ResourceProviderFailed {
        uri: String,
        #[source]
        source: ProviderError,
    },
    #[error("unable to serialize the output of tool '{tool_name}'")]
    SerializeToolOutput {
        tool_name: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("session '{session_id}' not found")]
    SessionNotFound { session_id: String },
    #[error("tool '{tool_name}' failed")]
    ToolFailed {
        tool_name: String,
        #[source]
        source: ProviderError,
    },
    #[error("tool '{tool_name}' not found")]
    ToolNotFound { tool_name: String },
    #[error("initialize requests must not carry the Mcp-Session-Id header")]
    UnexpectedSessionHeader,
    #[error("unsupported protocol version '{received}'")]
    UnsupportedProtocolVersion { received: String },
}

impl ResponseError for McpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotAcceptable => StatusCode::NOT_ACCEPTABLE,
            Self::ResourceNotFound { .. } | Self::SessionNotFound { .. } => StatusCode::NOT_FOUND,
            Self::PromptProviderFailed { .. }
            | Self::ResourceProviderFailed { .. }
            | Self::SerializeToolOutput { .. }
            | Self::ToolFailed { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadySubscribed { .. }
            | Self::DeserializeToolInput { .. }
            | Self::EmptyListPage
            | Self::InvalidAcceptHeader { .. }
            | Self::InvalidResourceUri { .. }
            | Self::InvalidSessionHeader { .. }
            | Self::MissingProtocolVersionHeader
            | Self::MissingResourceUriAuthority { .. }
            | Self::MissingResourceUriScheme { .. }
            | Self::MissingSessionHeader
            | Self::NoResourceProvider { .. }
            | Self::ParseMessage { .. }
            | Self::PromptNotFound { .. }
            | Self::ReadPayload { .. }
            | Self::ToolNotFound { .. }
            | Self::UnexpectedSessionHeader
            | Self::UnsupportedProtocolVersion { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(JsonRpcErrorResponse::from_mcp_error(None, self))
    }
}
