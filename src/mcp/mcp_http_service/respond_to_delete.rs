use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::MCP_SESSION_HEADER_NAME;
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
            session,
            session_manager,
            ..
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>> {
        match session {
            Some(session) => {
                session_manager.terminate_session(session).await?;

                Ok(HttpResponse::Accepted().into())
            }
            None => Ok(HttpResponse::BadRequest().body(format!(
                "You need to specify {MCP_SESSION_HEADER_NAME} header to use this endpoint"
            ))),
        }
    }
}
