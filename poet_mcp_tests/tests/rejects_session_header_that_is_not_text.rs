use actix_web::http::header::HeaderValue;
use poet_mcp::mcp_error::McpError;
use poet_mcp::session_manager::SessionManager;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::session_headers::session_headers;

#[test]
fn rejects_session_header_that_is_not_text() -> Result<(), PoetMcpTestsError> {
    assert!(matches!(
        SessionManager::default()
            .request_session(&session_headers(HeaderValue::from_bytes(&[0xff])?)),
        Err(McpError::InvalidSessionHeader { .. })
    ));

    Ok(())
}
