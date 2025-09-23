use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;

#[derive(Clone)]
pub struct RespondToGet {}

#[async_trait(?Send)]
impl McpResponder for RespondToGet {
    fn accepts() -> Vec<Mime> {
        vec![mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(&self, _ctx: McpResponderContext) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok().body("hello, world, get".to_string()))
    }
}
