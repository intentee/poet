use std::pin::Pin;

use actix_web::Handler;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;

use crate::mcp::accepts_all::Conclusion;
use crate::mcp::accepts_all::accepts_all;
use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct McpResponderHandler<TResponder>(pub TResponder)
where
    TResponder: McpResponder + 'static;

impl<TResponder> Handler<(HttpRequest, Payload)> for McpResponderHandler<TResponder>
where
    TResponder: McpResponder + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Self::Output>>>;
    type Output = Result<HttpResponse<BoxBody>>;

    fn call(&self, (req, payload): (HttpRequest, Payload)) -> Self::Future {
        let this = self.clone();

        Box::pin(async move {
            match accepts_all(&req, TResponder::accepts()) {
                Conclusion::AllAcceptable => this.0.respond_to(req, payload).await,
                Conclusion::NotAllAcceptable => Ok(HttpResponse::NotAcceptable().into()),
                Conclusion::ErrorParsingHeader(err) => {
                    Ok(HttpResponse::InternalServerError().body(format!("{err:#}")))
                }
            }
        })
    }
}
