use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorBadRequest;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::MCP_HEADER_PROTOCOL_VERSION;
use crate::mcp::mcp_responder_context::McpResponderContext;
use crate::mcp::session::Session;

#[async_trait(?Send)]
pub trait McpResponder: Clone {
    fn accepts() -> Vec<Mime>;

    async fn respond_to(&self, context: McpResponderContext) -> Result<HttpResponse<BoxBody>>;

    fn assert_protocol_version_header(
        &self,
        req: &HttpRequest,
        expected_version: &str,
    ) -> Result<()> {
        match req.headers().get(MCP_HEADER_PROTOCOL_VERSION) {
            Some(header_protocol_version) => {
                if header_protocol_version == expected_version {
                    Ok(())
                } else {
                    Err(ErrorBadRequest(format!(
                        "Unsupported protocol version: {header_protocol_version:?}"
                    )))
                }
            }
            None => Err(ErrorBadRequest(format!(
                "You need to use {MCP_HEADER_PROTOCOL_VERSION} header"
            ))),
        }
    }

    fn assert_no_session(&self, session: &Option<Session>) -> Result<()> {
        if session.is_some() {
            Err(ErrorBadRequest(
                "Unexpected session headers. Do not use session with this JSONRPC method.",
            ))
        } else {
            Ok(())
        }
    }

    fn assert_session(&self, session: &Option<Session>) -> Result<()> {
        if session.is_some() {
            Ok(())
        } else {
            Err(ErrorBadRequest("Expected session headers."))
        }
    }
}
