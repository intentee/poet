use actix_web::test::TestRequest;
use poet_mcp::mcp_header_session::MCP_HEADER_SESSION;
use serde_json::Value;

use crate::mcp_request::mcp_request;

#[must_use]
pub fn mcp_session_request(session_id: &str, message: &Value) -> TestRequest {
    mcp_request(message).insert_header((MCP_HEADER_SESSION, session_id))
}
