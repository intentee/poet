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
use futures_util::future::LocalBoxFuture;

use crate::mcp_error::McpError;
use crate::mcp_responder::McpResponder as _;
use crate::mcp_responder_context::McpResponderContext;
use crate::mcp_server::McpServer;
use crate::respond_to_delete::RespondToDelete;
use crate::respond_to_post::RespondToPost;

fn responder_context(
    service_request: &mut ServiceRequest,
    mcp_server: &McpServer,
) -> Result<McpResponderContext, McpError> {
    Ok(McpResponderContext {
        request_session: mcp_server
            .session_manager
            .request_session(service_request.headers())?,
        payload: service_request.take_payload(),
        http_request: service_request.request().clone(),
    })
}

async fn respond(
    service_request: &mut ServiceRequest,
    mcp_server: McpServer,
) -> Result<HttpResponse<BoxBody>, McpError> {
    match *service_request.method() {
        Method::DELETE => {
            let context = responder_context(service_request, &mcp_server)?;

            RespondToDelete {
                session_manager: mcp_server.session_manager,
            }
            .respond(context)
            .await
        }
        Method::POST => {
            let context = responder_context(service_request, &mcp_server)?;

            RespondToPost { mcp_server }.respond(context).await
        }
        _ => Ok(HttpResponse::MethodNotAllowed()
            .insert_header(header::Allow(vec![Method::DELETE, Method::POST]))
            .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
            .body("Method not allowed")),
    }
}

pub struct McpHttpService {
    pub mcp_server: McpServer,
}

impl Service<ServiceRequest> for McpHttpService {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<BoxBody>;

    always_ready!();

    fn call(&self, mut service_request: ServiceRequest) -> Self::Future {
        let mcp_server = self.mcp_server.clone();

        Box::pin(async move {
            Ok(match respond(&mut service_request, mcp_server).await {
                Ok(http_response) => service_request.into_response(http_response),
                Err(mcp_error) => {
                    ServiceResponse::from_err(mcp_error, service_request.into_parts().0)
                }
            })
        })
    }
}
