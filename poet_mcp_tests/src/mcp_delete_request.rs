use actix_web::http::header::ACCEPT;
use actix_web::test::TestRequest;
use poet_mcp::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;
use poet_mcp::mcp_header_session::MCP_HEADER_SESSION;
use poet_mcp::mcp_protocol_version::MCP_PROTOCOL_VERSION;

use crate::mcp_test_path::MCP_TEST_PATH;

#[must_use]
pub fn mcp_delete_request(session_id: &str) -> TestRequest {
    TestRequest::delete()
        .uri(MCP_TEST_PATH)
        .insert_header((ACCEPT, "application/json"))
        .insert_header((MCP_HEADER_PROTOCOL_VERSION, MCP_PROTOCOL_VERSION))
        .insert_header((MCP_HEADER_SESSION, session_id))
}
