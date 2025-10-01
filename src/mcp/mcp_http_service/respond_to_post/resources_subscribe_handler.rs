use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Bytes;
use chrono::Utc;
use futures_core::stream::Stream;
use tokio::sync::mpsc::Receiver;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribe;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribeParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::server_to_client_message::ServerToClientMessage;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

fn notifications_stream(
    subscriber: Receiver<ResourceContentParts>,
) -> impl Stream<Item = Result<Bytes>> {
    async_stream::try_stream! {
        let data = format!("data: Event at {}\n\n", Utc::now());

        yield Bytes::from(data);
    }
}

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
            Some(subscriber) => Ok(HttpResponse::Ok()
                .content_type(mime::TEXT_EVENT_STREAM)
                .streaming(notifications_stream(subscriber))),
            None => Ok(HttpResponse::Ok()
                .insert_header((MCP_HEADER_SESSION, session.session_id))
                .json(ServerToClientMessage::Error(Error::resource_not_found(
                    id, uri,
                )))),
        }
    }
}
