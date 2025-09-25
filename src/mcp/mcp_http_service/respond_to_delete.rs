use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;

/// This method is only used to terminate the session
/// https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#session-management
#[derive(Clone)]
pub struct RespondToDelete {}

#[async_trait(?Send)]
impl McpResponder for RespondToDelete {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON]
    }

    async fn respond_to(
        &self,
        McpResponderContext {
            req,
            session,
            session_manager,
            ..
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>> {
        self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;

        match session {
            Some(session) => {
                session_manager.terminate_session(session).await?;

                Ok(HttpResponse::Accepted().into())
            }
            None => Ok(HttpResponse::Gone().body(format!(
                "Session already gone, or you need to specify {MCP_HEADER_SESSION} header to use this endpoint"
            ))),
        }
    }
}
