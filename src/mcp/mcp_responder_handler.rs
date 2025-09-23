use std::pin::Pin;

use actix_web::Handler;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;

use crate::mcp::accepts_all::Conclusion;
use crate::mcp::accepts_all::accepts_all;
use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;

type Args = (McpResponderContext,);

#[derive(Clone)]
pub struct McpResponderHandler<TResponder>(pub TResponder)
where
    TResponder: McpResponder + 'static;

impl<TResponder> Handler<Args> for McpResponderHandler<TResponder>
where
    TResponder: McpResponder + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Self::Output>>>;
    type Output = Result<HttpResponse<BoxBody>>;

    fn call(&self, (ctx,): Args) -> Self::Future {
        let this = self.clone();

        Box::pin(async move {
            match accepts_all(&ctx.req, TResponder::accepts()) {
                Conclusion::AllAcceptable => this.0.respond_to(ctx).await,
                Conclusion::NotAllAcceptable => Ok(HttpResponse::NotAcceptable().into()),
                Conclusion::ErrorParsingHeader(err) => {
                    Ok(HttpResponse::InternalServerError().body(format!("{err:#}")))
                }
            }
        })
    }
}
