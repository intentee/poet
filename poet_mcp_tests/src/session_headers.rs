use actix_web::http::header::HeaderMap;
use actix_web::http::header::HeaderValue;
use actix_web::test::TestRequest;
use poet_mcp::mcp_header_session::MCP_HEADER_SESSION;

#[must_use]
pub fn session_headers(session_header: HeaderValue) -> HeaderMap {
    TestRequest::default()
        .insert_header((MCP_HEADER_SESSION, session_header))
        .to_http_request()
        .headers()
        .clone()
}
