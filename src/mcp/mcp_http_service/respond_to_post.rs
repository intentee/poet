use actix_web::FromRequest as _;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;
use mime::Mime;

use crate::jsonrpc::JSONRPC_VERSION;
use crate::jsonrpc::client_to_server_message::ClientToServerMessage;
use crate::jsonrpc::empty_object::EmptyObject;
use crate::jsonrpc::implementation::Implementation;
use crate::jsonrpc::params_with_meta::ParamsWithMeta;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;
use crate::jsonrpc::request::initialize::InitializeParams;
use crate::jsonrpc::response::error::Error;
use crate::jsonrpc::response::success::Success;
use crate::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::jsonrpc::response::success::initialize_result::ServerCapabilities;
use crate::jsonrpc::server_to_client_message::ServerToClientMessage;
use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::mcp_responder::McpResponder;
use crate::mcp::mcp_responder_context::McpResponderContext;
use crate::mcp::session_manager::SessionManager;

#[derive(Clone)]
pub struct RespondToPost {
    pub server_info: Implementation,
    pub session_manager: SessionManager,
}

impl RespondToPost {
    fn empty_response<TParameters>(
        &self,
        Request { id, .. }: Request<TParameters>,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(
            HttpResponse::Ok().json(ServerToClientMessage::Pong(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: EmptyResponse {},
            })),
        )
    }

    async fn respond_to_initialize(
        &self,
        Request {
            id,
            payload:
                Initialize {
                    method: _,
                    params:
                        ParamsWithMeta {
                            params: InitializeParams { capabilities, .. },
                            ..
                        },
                },
            ..
        }: Request<Initialize>,
    ) -> Result<HttpResponse<BoxBody>> {
        println!("{capabilities:?}");

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
                        resources: None,
                        tools: None,
                    },
                    instructions: None,
                    protocol_version: MCP_PROTOCOL_VERSION.to_string(),
                    server_info: self.server_info.clone(),
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
            ..
        }: McpResponderContext,
    ) -> Result<HttpResponse<BoxBody>> {
        let client_to_server_message: ClientToServerMessage =
            match String::from_request(&req, &mut payload).await {
                Ok(string_payload) => match serde_json::from_str(&string_payload) {
                    Ok(client_to_server_message) => client_to_server_message,
                    Err(err) => {
                        return Ok(HttpResponse::BadRequest().json(Error::parse(format!(
                            "Parse error: {err:#}\nPayload: {string_payload}"
                        ))));
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
                self.assert_session(&session)?;
                self.empty_response(request)
            }
            ClientToServerMessage::Ping(request) => {
                self.assert_protocol_version_header(&req, MCP_PROTOCOL_VERSION)?;
                self.assert_session(&session)?;
                self.empty_response(request)
            }
        }
    }
}
