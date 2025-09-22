use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::mcp_responder::McpResponder;

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
        req: HttpRequest,
        payload: Payload,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok().body("hello, world, delete".to_string()))
    }
}
