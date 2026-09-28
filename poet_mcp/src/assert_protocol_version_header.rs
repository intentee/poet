use actix_web::HttpRequest;

use crate::mcp_error::McpError;
use crate::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;
use crate::mcp_protocol_version::MCP_PROTOCOL_VERSION;

pub fn assert_protocol_version_header(http_request: &HttpRequest) -> Result<(), McpError> {
    let protocol_version = http_request
        .headers()
        .get(MCP_HEADER_PROTOCOL_VERSION)
        .ok_or(McpError::MissingProtocolVersionHeader)?;

    if protocol_version == MCP_PROTOCOL_VERSION {
        Ok(())
    } else {
        Err(McpError::UnsupportedProtocolVersion {
            received: String::from_utf8_lossy(protocol_version.as_bytes()).into_owned(),
        })
    }
}
