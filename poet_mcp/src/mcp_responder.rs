use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::accepts_all::accepts_all;
use crate::mcp_error::McpError;
use crate::mcp_responder_context::McpResponderContext;

#[async_trait(?Send)]
pub trait McpResponder: Sized {
    fn accepts() -> Vec<Mime>;

    async fn respond_to(
        self,
        context: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>, McpError>;

    async fn respond(
        self,
        context: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>, McpError> {
        accepts_all(&context.http_request, &Self::accepts())?;

        self.respond_to(context).await
    }
}
