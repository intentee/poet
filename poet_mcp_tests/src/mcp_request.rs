use actix_web::http::header::ACCEPT;
use actix_web::test::TestRequest;
use poet_mcp::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;
use poet_mcp::mcp_protocol_version::MCP_PROTOCOL_VERSION;
use serde_json::Value;

use crate::mcp_test_path::MCP_TEST_PATH;

#[must_use]
pub fn mcp_request(message: &Value) -> TestRequest {
    TestRequest::post()
        .uri(MCP_TEST_PATH)
        .insert_header((ACCEPT, "application/json, text/event-stream"))
        .insert_header((MCP_HEADER_PROTOCOL_VERSION, MCP_PROTOCOL_VERSION))
        .set_json(message)
}
