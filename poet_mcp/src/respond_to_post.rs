use actix_web::FromRequest as _;
use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use log::warn;
use mime::Mime;
use serde_json::from_str;

use crate::assert_protocol_version_header::assert_protocol_version_header;
use crate::client_to_server_message::ClientToServerMessage;
use crate::initialize_handler::InitializeHandler;
use crate::initialized_handler::InitializedHandler;
use crate::json_rpc_error_response::JsonRpcErrorResponse;
use crate::logging_set_level_handler::LoggingSetLevelHandler;
use crate::mcp_error::McpError;
use crate::mcp_header_session::MCP_HEADER_SESSION;
use crate::mcp_responder::McpResponder;
use crate::mcp_responder_context::McpResponderContext;
use crate::mcp_server::McpServer;
use crate::ping_handler::PingHandler;
use crate::prompts_get_handler::PromptsGetHandler;
use crate::prompts_list_handler::PromptsListHandler;
use crate::request_id::RequestId;
use crate::request_session::RequestSession;
use crate::resources_list_handler::ResourcesListHandler;
use crate::resources_read_handler::ResourcesReadHandler;
use crate::resources_subscribe_handler::ResourcesSubscribeHandler;
use crate::resources_templates_list_handler::ResourcesTemplatesListHandler;
use crate::resources_unsubscribe_handler::ResourcesUnsubscribeHandler;
use crate::server_to_client_response::ServerToClientResponse;
use crate::session::Session;
use crate::tools_call_handler::ToolsCallHandler;
use crate::tools_list_handler::ToolsListHandler;

async fn respond_within_session<TRespond>(
    request_session: RequestSession,
    request_id: RequestId,
    respond: TRespond,
) -> Result<HttpResponse<BoxBody>, McpError>
where
    TRespond: AsyncFnOnce(&Session) -> Result<ServerToClientResponse, McpError>,
{
    let session = request_session.established()?;
    let server_to_client_response = respond(&session).await.unwrap_or_else(|mcp_error| {
        warn!("Unable to handle JSON-RPC request: {mcp_error}");

        ServerToClientResponse::Error(JsonRpcErrorResponse::from_mcp_error(
            Some(request_id),
            &mcp_error,
        ))
    });

    Ok(HttpResponse::Ok()
        .insert_header((MCP_HEADER_SESSION, session.id))
        .json(server_to_client_response))
}

pub struct RespondToPost {
    pub mcp_server: McpServer,
}

#[async_trait(?Send)]
impl McpResponder for RespondToPost {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON, mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        self,
        McpResponderContext {
            mut payload,
            http_request,
            request_session,
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>, McpError> {
        let request_payload = String::from_request(&http_request, &mut payload)
            .await
            .map_err(|source| McpError::ReadPayload { source })?;
        let client_to_server_message: ClientToServerMessage =
            from_str(&request_payload).map_err(|source| McpError::ParseMessage { source })?;

        if !matches!(
            client_to_server_message,
            ClientToServerMessage::Initialize(_)
        ) {
            assert_protocol_version_header(&http_request)?;
        }

        let McpServer {
            prompt_provider,
            resource_list_aggregate,
            server_info,
            session_manager,
            tool_registry,
        } = self.mcp_server;

        match client_to_server_message {
            ClientToServerMessage::Initialize(initialize_request) => {
                request_session.require_absent()?;

                Ok(InitializeHandler {
                    server_info,
                    session_manager,
                }
                .handle(initialize_request))
            }
            ClientToServerMessage::Initialized(initialized_notification) => Ok(InitializedHandler
                .handle(initialized_notification, &request_session.established()?)
                .await),
            ClientToServerMessage::LoggingSetLevel(request) => {
                respond_within_session(request_session, request.id.clone(), async |session| {
                    Ok(LoggingSetLevelHandler { session_manager }.handle(request, session))
                })
                .await
            }
            ClientToServerMessage::Ping(request) => {
                Ok(HttpResponse::Ok().json(PingHandler.handle(request)))
            }
            ClientToServerMessage::PromptsGet(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    PromptsGetHandler { prompt_provider }.handle(request).await
                })
                .await
            }
            ClientToServerMessage::PromptsList(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    PromptsListHandler { prompt_provider }.handle(request).await
                })
                .await
            }
            ClientToServerMessage::ResourcesList(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    ResourcesListHandler {
                        resource_list_aggregate,
                    }
                    .handle(request)
                    .await
                })
                .await
            }
            ClientToServerMessage::ResourcesRead(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    ResourcesReadHandler {
                        resource_list_aggregate,
                    }
                    .handle(request)
                    .await
                })
                .await
            }
            ClientToServerMessage::ResourcesSubscribe(request) => {
                respond_within_session(request_session, request.id.clone(), async |session| {
                    ResourcesSubscribeHandler {
                        resource_list_aggregate,
                    }
                    .handle(request, session)
                })
                .await
            }
            ClientToServerMessage::ResourcesTemplatesList(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    Ok(ResourcesTemplatesListHandler {
                        resource_list_aggregate,
                    }
                    .handle(request))
                })
                .await
            }
            ClientToServerMessage::ResourcesUnsubscribe(request) => {
                respond_within_session(request_session, request.id.clone(), async |session| {
                    Ok(ResourcesUnsubscribeHandler.handle(request, session))
                })
                .await
            }
            ClientToServerMessage::ToolsCall(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    ToolsCallHandler { tool_registry }.handle(request).await
                })
                .await
            }
            ClientToServerMessage::ToolsList(request) => {
                respond_within_session(request_session, request.id.clone(), async |_| {
                    Ok(ToolsListHandler { tool_registry }.handle(request))
                })
                .await
            }
        }
    }
}
