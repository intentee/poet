use actix_web::test::TestRequest;
use poet_mcp::assert_protocol_version_header::assert_protocol_version_header;
use poet_mcp::mcp_error::McpError;
use poet_mcp::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;

#[test]
fn rejects_unsupported_protocol_version() {
    assert!(matches!(
        assert_protocol_version_header(
            &TestRequest::default()
                .insert_header((MCP_HEADER_PROTOCOL_VERSION, "2024-11-05"))
                .to_http_request()
        ),
        Err(McpError::UnsupportedProtocolVersion { received }) if received == "2024-11-05"
    ));
}
