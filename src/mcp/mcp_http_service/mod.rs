pub mod respond_to_delete;
pub mod respond_to_get;
pub mod respond_to_post;

use std::sync::Arc;

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

use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::mcp_http_service::respond_to_delete::RespondToDelete;
use crate::mcp::mcp_http_service::respond_to_get::RespondToGet;
use crate::mcp::mcp_http_service::respond_to_post::RespondToPost;
use crate::mcp::mcp_responder_context::McpResponderContext;
use crate::mcp::mcp_responder_handler::McpResponderHandler;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session_manager::SessionManager;
use crate::mcp::tool_registry::ToolRegistry;

pub struct McpHttpService {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
    pub server_info: Implementation,
    pub session_manager: SessionManager,
    pub tool_registry: Arc<ToolRegistry>,
}

impl Service<ServiceRequest> for McpHttpService {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<BoxBody>;

    always_ready!();

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let req_method = req.method().clone();
        let resource_list_aggregate = self.resource_list_aggregate.clone();
        let server_info = self.server_info.clone();
        let session_manager = self.session_manager.clone();
        let tool_registry = self.tool_registry.clone();

        Box::pin(async move {
            let ctx = McpResponderContext {
                payload: req.take_payload(),
                req: req.request().clone(),
                session: session_manager.restore_session(&req).await?,
                session_manager: session_manager.clone(),
            };

            let http_response = match req_method {
                Method::DELETE => McpResponderHandler(RespondToDelete {}).call((ctx,)).await?,
                Method::GET => McpResponderHandler(RespondToGet {}).call((ctx,)).await?,
                Method::POST => {
                    McpResponderHandler(RespondToPost {
                        resource_list_aggregate,
                        server_info,
                        session_manager,
                        tool_registry,
                    })
                    .call((ctx,))
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
