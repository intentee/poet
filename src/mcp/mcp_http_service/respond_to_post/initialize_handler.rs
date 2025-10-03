use std::time::Duration;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::web::Bytes;
use futures_core::stream::Stream;
use log::error;
use tokio::sync::mpsc::Receiver;
use tokio::time::interval;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::mcp::jsonrpc::response::success::initialize_result::ServerCapabilities;
use crate::mcp::jsonrpc::response::success::initialize_result::ServerCapabilityResources;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::session::Session;
use crate::mcp::session_manager::SessionManager;
use crate::mcp::session_with_notifications_receiver::SessionWithNotificationsReceiver;

#[derive(Clone)]
pub struct InitializeHandler {
    pub server_info: Implementation,
    pub session_manager: SessionManager,
}

impl InitializeHandler {
    fn notifications_stream(
        self,
        id: Id,
        mut notification_rx: Receiver<ServerToClientNotification>,
        session: Session,
    ) -> impl Stream<Item = Result<Bytes>> {
        async_stream::try_stream! {
            let confirmation = ServerToClientResponse::InitializeResult(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: InitializeResult {
                    capabilities: ServerCapabilities {
                        completions: None,
                        experimental: None,
                        logging: Some(EmptyObject {}),
                        prompts: None,
                        resources: Some(ServerCapabilityResources {
                            list_changed: true,
                            subscribe: true,
                        }),
                        tools: None,
                    },
                    instructions: None,
                    protocol_version: MCP_PROTOCOL_VERSION.to_string(),
                    server_info: self.server_info,
                },
            });

            match serde_json::to_string(&confirmation) {
                Ok(confirmation_serialized) => {
                    yield Bytes::from(format!("data: {confirmation_serialized}\n\n"));
                },
                Err(err) => {
                    error!("Unable to serialize initialize confirmation: {err:#?}");

                    return;
                }
            }

            let mut ticker = interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    resource_content_parts = notification_rx.recv() => {
                        match resource_content_parts {
                            Some(resource_content_parts) => {
                                match serde_json::to_string(&resource_content_parts) {
                                    Ok(serialized) => yield Bytes::from(format!("data: {serialized}\n\n")),
                                    Err(err) => {
                                        error!("{err}");

                                        yield Bytes::from(": server-error\n\n");
                                    }
                                }
                            }
                            None => break,
                        }
                    }
                    _ = ticker.tick() => {
                        yield Bytes::from(": keep-alive\n\n");
                    }
                }
            }

            notification_rx.close();

            if let Err(err) = self.session_manager.terminate_session(session).await {
                error!("Unable to terminate session: {err:#?}");
            }
        }
    }
}

impl InitializeHandler {
    pub async fn handle(self, Initialize { id, .. }: Initialize) -> Result<HttpResponse<BoxBody>> {
        let SessionWithNotificationsReceiver {
            notification_rx,
            session,
        } = self.session_manager.start_new_session().await?;

        Ok(HttpResponse::Ok()
            .content_type(mime::TEXT_EVENT_STREAM)
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .streaming(self.notifications_stream(id, notification_rx, session)))
    }
}
