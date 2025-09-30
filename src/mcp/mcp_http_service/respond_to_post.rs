use std::sync::Arc;

use actix_web::FromRequest as _;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;
use log::error;
use log::warn;
use mime::Mime;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::client_to_server_message::ClientToServerMessage;
use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::request::initialize::InitializeParams;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevelParams;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList as ResourcesListRequest;
use crate::mcp::jsonrpc::request::resources_list::ResourcesListParams;
use crate::mcp::jsonrpc::request::resources_read::ResourcesRead as ResourcesReadRequest;
use crate::mcp::jsonrpc::request::resources_read::ResourcesReadParams;
use crate::mcp::jsonrpc::request::resources_templates_list::ResourcesTemplatesList as ResourcesTemplatesListRequest;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::mcp::jsonrpc::response::success::initialize_result::ServerCapabilities;
use crate::mcp::jsonrpc::response::success::initialize_result::ServerCapabilityResources;
use crate::mcp::jsonrpc::response::success::resource_templates_list::ResourcesTemplatesList as ResourcesTemplatesListResponse;
use crate::mcp::jsonrpc::response::success::resources_list::ResourcesList as ResourcesListResponse;
use crate::mcp::jsonrpc::response::success::resources_read::ResourcesRead as ResourcesReadResponse;
use crate::mcp::jsonrpc::server_to_client_message::ServerToClientMessage;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;
use crate::mcp::session_manager::SessionManager;

const PER_PAGE: usize = 100;

#[derive(Clone)]
pub struct RespondToPost {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
    pub server_info: Implementation,
    pub session_manager: SessionManager,
}

impl RespondToPost {
    fn empty_response(&self, request_id: Id) -> Result<HttpResponse<BoxBody>> {
        Ok(
            HttpResponse::Ok().json(ServerToClientMessage::EmptyResponse(Success {
                id: request_id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: EmptyResponse {},
            })),
        )
    }

    async fn respond_to_initialize(
        &self,
        Initialize {
            id,
            params: InitializeParams { .. },
            ..
        }: Initialize,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok()
            .insert_header((
                MCP_HEADER_SESSION,
                self.session_manager.start_new_session().await?.session_id,
            ))
            .json(ServerToClientMessage::InitializeResult(Success {
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
                    server_info: self.server_info.clone(),
                },
            })))
    }

    async fn respond_to_logging_set_level(
        &self,
        LoggingSetLevel {
            id,
            params: LoggingSetLevelParams { level, .. },
            ..
        }: LoggingSetLevel,
        session: Session,
        session_manager: SessionManager,
    ) -> Result<HttpResponse<BoxBody>> {
        session_manager
            .update_session(session.with_log_level(level))
            .await?;
        self.empty_response(id)
    }

    async fn respond_to_resources_list(
        &self,
        ResourcesListRequest {
            id,
            params: ResourcesListParams { cursor, .. },
            ..
        }: ResourcesListRequest,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok()
            .insert_header((
                MCP_HEADER_SESSION,
                self.session_manager.start_new_session().await?.session_id,
            ))
            .json(ServerToClientMessage::ResourcesList(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: ResourcesListResponse {
                    resources: self
                        .resource_list_aggregate
                        .list_resources(ListResourcesParams {
                            cursor: cursor.unwrap_or_default(),
                            per_page: PER_PAGE,
                        })
                        .await
                        .map_err(ErrorInternalServerError)?,
                },
            })))
    }

    async fn respond_to_resources_read(
        &self,
        ResourcesReadRequest {
            id,
            params: ResourcesReadParams { uri, .. },
            ..
        }: ResourcesReadRequest,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok()
            .insert_header((
                MCP_HEADER_SESSION,
                self.session_manager.start_new_session().await?.session_id,
            ))
            .json(
                match self
                    .resource_list_aggregate
                    .read_resource_contents(&uri)
                    .await
                    .map_err(ErrorInternalServerError)?
                {
                    Some(contents) => ServerToClientMessage::ResourcesRead(Success {
                        id,
                        jsonrpc: JSONRPC_VERSION.to_string(),
                        result: ResourcesReadResponse { contents },
                    }),
                    None => {
                        warn!("Resource not found: '{uri}'");

                        ServerToClientMessage::Error(Error::resource_not_found(id, uri))
                    }
                },
            ))
    }

    async fn respond_to_resources_templates_list(
        &self,
        ResourcesTemplatesListRequest { id, .. }: ResourcesTemplatesListRequest,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok()
            .insert_header((
                MCP_HEADER_SESSION,
                self.session_manager.start_new_session().await?.session_id,
            ))
            .json(ServerToClientMessage::ResourcesTemplatesList(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: ResourcesTemplatesListResponse {
                    resource_templates: self
                        .resource_list_aggregate
                        .read_resources_templates_list()
                        .await
                        .map_err(ErrorInternalServerError)?,
                },
            })))
    }
}

#[async_trait(?Send)]
impl McpResponder for RespondToPost {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON, mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        &self,
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

        match client_to_server_message {
            ClientToServerMessage::Initialize(request) => {
                self.assert_no_session(&session)?;
                self.respond_to_initialize(request).await
            }
            ClientToServerMessage::Initialized(_) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;

                Ok(HttpResponse::Accepted().into())
            }
            ClientToServerMessage::LoggingSetLevel(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.respond_to_logging_set_level(
                    request,
                    self.assert_session(&session)?.clone(),
                    session_manager,
                )
                .await
            }
            ClientToServerMessage::Ping(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.empty_response(request.id)
            }
            ClientToServerMessage::ResourcesList(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.respond_to_resources_list(request).await
            }
            ClientToServerMessage::ResourcesRead(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.respond_to_resources_read(request).await
            }
            ClientToServerMessage::ResourcesTemplatesList(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.respond_to_resources_templates_list(request).await
            }
        }
    }
}
