use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::assert_protocol_version_header::assert_protocol_version_header;
use crate::mcp_error::McpError;
use crate::mcp_responder::McpResponder;
use crate::mcp_responder_context::McpResponderContext;
use crate::session_manager::SessionManager;

pub struct RespondToDelete {
    pub session_manager: SessionManager,
}

#[async_trait(?Send)]
impl McpResponder for RespondToDelete {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON]
    }

    async fn respond_to(
        self,
        McpResponderContext {
            req,
            request_session,
            ..
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>, McpError> {
        assert_protocol_version_header(&req)?;

        self.session_manager
            .terminate_session(&request_session.established()?.id);

        Ok(HttpResponse::Accepted().finish())
    }
}
