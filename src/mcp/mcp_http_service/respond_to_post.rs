use actix_web::FromRequest as _;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::web::Json;
use async_trait::async_trait;
use mime::Mime;

use crate::jsonrpc::message::Message;
use crate::jsonrpc::response::error::Error;
use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct RespondToPost {}

#[async_trait(?Send)]
impl McpResponder for RespondToPost {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON, mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        &self,
        req: HttpRequest,
        mut payload: Payload,
    ) -> Result<HttpResponse<BoxBody>> {
        let json: Message = Json::<Message>::from_request(&req, &mut payload)
            .await?
            .into_inner();
        println!("{json:?}");

        match json {
            Message::Notification(_) | Message::Response(_) => {
                Ok(HttpResponse::BadRequest().json(Error::invalid_request()))
            }
            Message::Request(_) => Ok(HttpResponse::Ok().body("hello, world, post".to_string())),
        }
    }
}
