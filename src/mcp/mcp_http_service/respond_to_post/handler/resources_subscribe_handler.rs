use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use actix_web::rt;
use async_trait::async_trait;
use log::error;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdated;
use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdatedParams;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribe;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribeParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

#[derive(Clone)]
pub struct ResourcesSubscribeHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

#[async_trait]
impl Handler for ResourcesSubscribeHandler {
    type Request = ResourcesSubscribe;
    type Session = Session;

    async fn handle(
        self,
        ResourcesSubscribe {
            id,
            params: ResourcesSubscribeParams { uri, .. },
            ..
        }: ResourcesSubscribe,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let cancellation_token = session
            .subscribe_to(&uri)
            .await
            .map_err(ErrorInternalServerError)?;
        let cancellation_token_clone = cancellation_token.clone();

        match self
            .resource_list_aggregate
            .subscribe(cancellation_token, &uri)
            .await
            .map_err(ErrorInternalServerError)?
        {
            Some(mut resource_content_parts_rx) => {
                let session_clone = session.clone();

                rt::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = cancellation_token_clone.cancelled() => {
                                break;
                            }
                            resource_content_parts = resource_content_parts_rx.recv() => {
                                if let Some(ResourceContentParts {
                                    parts: _,
                                    title,
                                    uri,
                                }) = resource_content_parts {
                                    if let Err(err) = session_clone
                                        .notify(ServerToClientNotification::ResourcesUpdated(
                                            ResourcesUpdated {
                                                jsonrpc: JSONRPC_VERSION.to_string(),
                                                params: ResourcesUpdatedParams { title, uri },
                                            },
                                        ))
                                        .await
                                    {
                                        error!("Unable to send session notification: {err:#?}");
                                        cancellation_token_clone.cancel();
                                        break;
                                    }
                                } else {
                                    cancellation_token_clone.cancel();
                                    break;
                                }
                            }
                        }
                    }

                    resource_content_parts_rx.close();
                });

                self.empty_response(id, session)
            }
            None => Ok(HttpResponse::NotFound()
                .insert_header((MCP_HEADER_SESSION, session.id()))
                .json(ServerToClientResponse::Error(Error::resource_not_found(
                    id, uri,
                )))),
        }
    }
}
