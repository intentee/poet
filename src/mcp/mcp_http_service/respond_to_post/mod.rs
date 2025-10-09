mod handler;

use std::sync::Arc;

use actix_web::FromRequest as _;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use log::error;
use mime::Mime;

use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::jsonrpc::client_to_server_message::ClientToServerMessage;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler as _;
use crate::mcp::mcp_http_service::respond_to_post::handler::initialize_handler::InitializeHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::initialized_handler::InitializedHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::logging_set_level_handler::LoggingSetLevelHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::ping_handler::PingHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::prompts_list_handler::PromptsListHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::resources_list_handler::ResourcesListHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::resources_read_handler::ResourcesReadHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::resources_subscribe_handler::ResourcesSubscribeHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::resources_templates_list_handler::ResourcesTemplatesListHandler;
use crate::mcp::mcp_http_service::respond_to_post::handler::resources_unsubscribe_handler::ResourcesUnsubscribeHandler;
use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session_manager::SessionManager;

#[derive(Clone)]
pub struct RespondToPost {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
    pub server_info: Implementation,
    pub session_manager: SessionManager,
}

#[async_trait(?Send)]
impl McpResponder for RespondToPost {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON, mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        self,
        McpResponderContext {
            req,
            mut payload,
            session,
            session_manager,
            ..
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>> {
        let client_to_server_message: ClientToServerMessage =
            match String::from_request(&req, &mut payload).await {
                Ok(string_payload) => match serde_json::from_str(&string_payload) {
                    Ok(client_to_server_message) => client_to_server_message,
                    Err(err) => {
                        let message = format!("Parse error: {err:#}\nPayload: {string_payload}");

                        error!("{message}");

                        return Ok(HttpResponse::BadRequest().json(Error::parse(message)));
                    }
                },
                Err(err) => {
                    return Ok(
                        HttpResponse::BadRequest().json(Error::invalid_request(format!(
                            "No deserializable string payload: {err:#}"
                        ))),
                    );
                }
            };

        self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;

        match client_to_server_message {
            ClientToServerMessage::Initialize(request) => {
                self.assert_no_session(&session)?;

                InitializeHandler {
                    server_info: self.server_info,
                    session_manager: self.session_manager,
                }
                .handle(request, ())
                .await
            }
            ClientToServerMessage::Initialized(request) => {
                let session = self.assert_session(&session)?;

                InitializedHandler {}.handle(request, session).await
            }
            ClientToServerMessage::LoggingSetLevel(request) => {
                let session = self.assert_session(&session)?;

                LoggingSetLevelHandler { session_manager }
                    .handle(request, session)
                    .await
            }
            ClientToServerMessage::Ping(request) => PingHandler {}.handle(request, ()).await,
            ClientToServerMessage::PromptsList(request) => {
                let session = self.assert_session(&session)?;

                PromptsListHandler {}.handle(request, session).await
            }
            ClientToServerMessage::ResourcesList(request) => {
                let session = self.assert_session(&session)?;

                ResourcesListHandler {
                    resource_list_aggregate: self.resource_list_aggregate,
                }
                .handle(request, session)
                .await
            }
            ClientToServerMessage::ResourcesRead(request) => {
                let session = self.assert_session(&session)?;

                ResourcesReadHandler {
                    resource_list_aggregate: self.resource_list_aggregate,
                }
                .handle(request, session)
                .await
            }
            ClientToServerMessage::ResourcesSubscribe(request) => {
                let session = self.assert_session(&session)?;

                ResourcesSubscribeHandler {
                    resource_list_aggregate: self.resource_list_aggregate,
                }
                .handle(request, session)
                .await
            }
            ClientToServerMessage::ResourcesTemplatesList(request) => {
                let session = self.assert_session(&session)?;

                ResourcesTemplatesListHandler {
                    resource_list_aggregate: self.resource_list_aggregate,
                }
                .handle(request, session)
                .await
            }
            ClientToServerMessage::ResourcesUnsubscribe(request) => {
                let session = self.assert_session(&session)?;

                ResourcesUnsubscribeHandler {}
                    .handle(request, session)
                    .await
            }
        }
    }
}
