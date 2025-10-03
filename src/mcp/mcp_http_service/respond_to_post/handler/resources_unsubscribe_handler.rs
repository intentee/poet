use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;

use crate::mcp::jsonrpc::request::resources_unsubscribe::ResourcesUnsubscribe;
use crate::mcp::jsonrpc::request::resources_unsubscribe::ResourcesUnsubscribeParams;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;

pub struct ResourcesUnsubscribeHandler {}

#[async_trait]
impl Handler for ResourcesUnsubscribeHandler {
    type Request = ResourcesUnsubscribe;
    type Session = Session;

    async fn handle(
        self,
        ResourcesUnsubscribe {
            id,
            params: ResourcesUnsubscribeParams { uri, .. },
            ..
        }: ResourcesUnsubscribe,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        if let Some(cancallation_token) = session
            .subscribe_token(&uri)
            .map_err(ErrorInternalServerError)?
        {
            cancallation_token.cancel();
        }

        self.empty_response(id, session)
    }
}
