use std::time::Duration;

use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use actix_web::web::Bytes;
use async_stream::stream;
use futures_core::stream::Stream;
use log::warn;
use serde::Serialize;
use serde_json::to_string;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::interval;

use crate::empty_object::EmptyObject;
use crate::implementation::Implementation;
use crate::initialize_request::InitializeRequest;
use crate::initialize_result::InitializeResult;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_header_session::MCP_HEADER_SESSION;
use crate::mcp_protocol_version::MCP_PROTOCOL_VERSION;
use crate::request_id::RequestId;
use crate::server_capabilities::ServerCapabilities;
use crate::server_capability_prompts::ServerCapabilityPrompts;
use crate::server_capability_resources::ServerCapabilityResources;
use crate::server_capability_tools::ServerCapabilityTools;
use crate::server_to_client_notification::ServerToClientNotification;
use crate::server_to_client_response::ServerToClientResponse;
use crate::session_manager::SessionManager;
use crate::session_with_notifications_receiver::SessionWithNotificationsReceiver;

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(1);

fn server_sent_event_data<TData: Serialize>(data: &TData) -> Result<Bytes, serde_json::Error> {
    to_string(data).map(|serialized_data| Bytes::from(format!("data: {serialized_data}\n\n")))
}

pub struct InitializeHandler {
    pub server_info: Implementation,
    pub session_manager: SessionManager,
}

impl InitializeHandler {
    #[must_use]
    pub fn handle(
        self,
        InitializeRequest { id, params, .. }: InitializeRequest,
    ) -> HttpResponse<BoxBody> {
        if !params.capabilities.extra.is_empty() {
            warn!(
                "Unknown fields in client capabilities: {:#?}",
                params.capabilities.extra
            );
        }

        let SessionWithNotificationsReceiver {
            notification_rx,
            session,
        } = self.session_manager.start_new_session();

        HttpResponse::Ok()
            .content_type(mime::TEXT_EVENT_STREAM)
            .insert_header((MCP_HEADER_SESSION, session.id))
            .streaming(self.notifications_stream(id, notification_rx))
    }

    fn notifications_stream(
        self,
        id: RequestId,
        mut notification_rx: Receiver<ServerToClientNotification>,
    ) -> impl Stream<Item = Result<Bytes, serde_json::Error>> {
        stream! {
            let confirmation = ServerToClientResponse::InitializeResult(JsonRpcSuccessResponse::new(
                id,
                InitializeResult {
                    capabilities: ServerCapabilities {
                        completions: None,
                        experimental: None,
                        logging: Some(EmptyObject {}),
                        prompts: Some(ServerCapabilityPrompts { list_changed: true }),
                        resources: Some(ServerCapabilityResources {
                            list_changed: true,
                            subscribe: true,
                        }),
                        tools: Some(ServerCapabilityTools { list_changed: true }),
                    },
                    instructions: None,
                    protocol_version: MCP_PROTOCOL_VERSION.to_owned(),
                    server_info: self.server_info,
                },
            ));

            yield server_sent_event_data(&confirmation);

            let mut keep_alive_ticker = interval(KEEP_ALIVE_INTERVAL);

            loop {
                let server_sent_event = select! {
                    notification = notification_rx.recv() => {
                        let Some(notification) = notification else {
                            break;
                        };

                        server_sent_event_data(&notification)
                    }
                    _ = keep_alive_ticker.tick() => Ok(Bytes::from_static(b": keep-alive\n\n")),
                };

                yield server_sent_event;
            }
        }
    }
}
