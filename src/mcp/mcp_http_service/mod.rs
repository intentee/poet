pub mod respond_to_delete;
pub mod respond_to_get;
pub mod respond_to_post;

use actix_web::Handler as _;
use actix_web::HttpMessage as _;
use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use actix_web::dev::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::always_ready;
use actix_web::error::Error;
use actix_web::http::Method;
use actix_web::http::header;
use actix_web::mime;
use futures_util::future::LocalBoxFuture;

use crate::jsonrpc::implementation::Implementation;
use crate::mcp::mcp_http_service::respond_to_delete::RespondToDelete;
use crate::mcp::mcp_http_service::respond_to_get::RespondToGet;
use crate::mcp::mcp_http_service::respond_to_post::RespondToPost;
use crate::mcp::mcp_responder_handler::McpResponderHandler;

pub struct McpHttpService {
    pub server_info: Implementation,
}

impl Service<ServiceRequest> for McpHttpService {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<BoxBody>;

    always_ready!();

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let req_method = req.method().clone();
        let server_info = self.server_info.clone();

        Box::pin(async move {
            let args = (req.request().clone(), req.take_payload());

            let http_response = match req_method {
                Method::DELETE => McpResponderHandler(RespondToDelete {}).call(args).await?,
                Method::GET => McpResponderHandler(RespondToGet {}).call(args).await?,
                Method::POST => {
                    McpResponderHandler(RespondToPost { server_info })
                        .call(args)
                        .await?
                }
                _ => HttpResponse::MethodNotAllowed()
                    .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
                    .body("Method not allowed"),
            };

            Ok(req.into_response(http_response))
        })
    }
}
