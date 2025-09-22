use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use async_trait::async_trait;
use mime::Mime;

use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct RespondToGet {}

#[async_trait(?Send)]
impl McpResponder for RespondToGet {
    fn accepts() -> Vec<Mime> {
        vec![mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        &self,
        req: HttpRequest,
        payload: Payload,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok().body("hello, world, get".to_string()))
    }
}
