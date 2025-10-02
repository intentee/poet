use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use actix_web::rt;
use log::error;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdated;
use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdatedParams;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribe;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribeParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

#[derive(Clone)]
pub struct ResourcesSubscribeHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

impl ResourcesSubscribeHandler {
    pub async fn handle(
        self,
        ResourcesSubscribe {
            id,
            params: ResourcesSubscribeParams { uri, .. },
            ..
        }: ResourcesSubscribe,
        session: Session,
    ) -> Result<HttpResponse<BoxBody>> {
        match self
            .resource_list_aggregate
            .subscribe(&uri)
            .await
            .map_err(ErrorInternalServerError)?
        {
            Some(mut subscriber) => {
                rt::spawn(async move {
                    while let Some(ResourceContentParts {
                        parts: _,
                        title,
                        uri,
                    }) = subscriber.recv().await
                    {
                        if let Err(err) = session
                            .notification_tx
                            .send(ServerToClientNotification::ResourcesUpdated(
                                ResourcesUpdated {
                                    jsonrpc: JSONRPC_VERSION.to_string(),
                                    params: ResourcesUpdatedParams { title, uri },
                                },
                            ))
                            .await
                        {
                            error!("Unable to send session notification: {err:#?}");
                            break;
                        }
                    }
                });

                Ok(HttpResponse::Ok()
                    .content_type(mime::TEXT_EVENT_STREAM)
                    .json(ServerToClientResponse::EmptyResponse(Success {
                        id,
                        jsonrpc: JSONRPC_VERSION.to_string(),
                        result: EmptyResponse {},
                    })))
            }
            None => Ok(HttpResponse::NotFound()
                .insert_header((MCP_HEADER_SESSION, session.session_id))
                .json(ServerToClientResponse::Error(Error::resource_not_found(
                    id, uri,
                )))),
        }
    }
}
