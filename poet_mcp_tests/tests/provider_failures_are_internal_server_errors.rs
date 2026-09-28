use std::io::Error as IoError;
use std::io::ErrorKind;

use actix_web::ResponseError as _;
use actix_web::http::StatusCode;
use poet_mcp::mcp_error::McpError;

#[test]
fn provider_failures_are_internal_server_errors() {
    assert_eq!(
        McpError::PromptProviderFailed {
            source: IoError::from(ErrorKind::NotConnected).into(),
        }
        .status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
