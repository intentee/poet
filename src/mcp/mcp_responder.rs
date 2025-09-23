use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::mcp_responder_context::McpResponderContext;

#[async_trait(?Send)]
pub trait McpResponder: Clone {
    fn accepts() -> Vec<Mime>;

    async fn respond_to(&self, context: McpResponderContext) -> Result<HttpResponse<BoxBody>>;
}
