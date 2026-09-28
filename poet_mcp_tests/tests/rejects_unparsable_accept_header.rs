use actix_web::http::header::HeaderValue;
use poet_mcp::mcp_error::McpError;
use poet_mcp_tests::accepts_all_with_header::accepts_all_with_header;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[test]
fn rejects_unparsable_accept_header() -> Result<(), PoetMcpTestsError> {
    assert!(matches!(
        accepts_all_with_header(HeaderValue::from_bytes(&[0xff])?, &[mime::APPLICATION_JSON]),
        Err(McpError::InvalidAcceptHeader { .. })
    ));

    Ok(())
}
