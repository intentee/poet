use std::pin::Pin;

use actix_web::Handler;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::body::BoxBody;

use crate::mcp::accepts_all::Conclusion;
use crate::mcp::accepts_all::accepts_all;
use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct McpResponderHandler<TResponder>(pub TResponder)
where
    TResponder: McpResponder + 'static;

impl<TResponder> McpResponderHandler<TResponder>
where
    TResponder: McpResponder + 'static,
{
    async fn respond_to(&self, req: HttpRequest) -> HttpResponse<BoxBody> {
        match self.0.respond_to(req).await {
            Ok(res) => res,
            Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
        }
    }
}

impl<TResponder> Handler<(HttpRequest,)> for McpResponderHandler<TResponder>
where
    TResponder: McpResponder + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Self::Output>>>;
    type Output = HttpResponse<BoxBody>;

    fn call(&self, (req,): (HttpRequest,)) -> Self::Future {
        let this = self.clone();

        Box::pin(async move {
            match accepts_all(&req, TResponder::accepts()) {
                Conclusion::AllAcceptable => this.respond_to(req).await,
                Conclusion::NotAllAcceptable => HttpResponse::NotAcceptable().into(),
                Conclusion::ErrorParsingHeader(err) => {
                    HttpResponse::InternalServerError().body(format!("{err}"))
                }
            }
        })
    }
}
