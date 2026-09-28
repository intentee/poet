use actix_web::http::header::ACCEPT;
use actix_web::http::header::HeaderValue;
use actix_web::test::TestRequest;
use mime::Mime;
use poet_mcp::accepts_all::accepts_all;
use poet_mcp::mcp_error::McpError;

pub fn accepts_all_with_header(
    accept_header: HeaderValue,
    required_mimes: &[Mime],
) -> Result<(), McpError> {
    accepts_all(
        &TestRequest::default()
            .insert_header((ACCEPT, accept_header))
            .to_http_request(),
        required_mimes,
    )
}
