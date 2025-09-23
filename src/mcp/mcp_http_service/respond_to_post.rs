use actix_web::FromRequest as _;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::web::Json;
use async_trait::async_trait;
use mime::Mime;

use crate::jsonrpc::JSONRPC_VERSION;
use crate::jsonrpc::client_to_server_message::ClientToServerMessage;
use crate::jsonrpc::implementation::Implementation;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;
use crate::jsonrpc::response::error::Error;
use crate::jsonrpc::response::success::Success;
use crate::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::jsonrpc::response::success::initialize_result::ServerCapabilities;
use crate::jsonrpc::response::success::pong::Pong;
use crate::jsonrpc::server_to_client_message::ServerToClientMessage;
use crate::mcp::MCP_PROTOCOL_VERSION;
use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct RespondToPost {
    pub server_info: Implementation,
}

#[async_trait(?Send)]
impl McpResponder for RespondToPost {
    fn accepts() -> Vec<Mime> {
        vec![mime::APPLICATION_JSON, mime::TEXT_EVENT_STREAM]
    }

    async fn respond_to(
        &self,
        req: HttpRequest,
        mut payload: Payload,
    ) -> Result<HttpResponse<BoxBody>> {
        // let json: Value = Json::<Value>::from_request(&req, &mut payload).await?.into_inner();
        // println!("raw: {json:?}");

        let client_to_server_message: ClientToServerMessage =
            match Json::<ClientToServerMessage>::from_request(&req, &mut payload).await {
                Ok(client_to_server_message) => client_to_server_message.into_inner(),
                Err(err) => {
                    return Ok(
                        HttpResponse::BadRequest().json(Error::invalid_request(format!("{err:#}")))
                    );
                }
            };

        println!("{client_to_server_message:?}");

        match client_to_server_message {
            ClientToServerMessage::Initialize(Request {
                id,
                jsonrpc,
                payload: Initialize { method, params },
            }) => Ok(
                HttpResponse::Ok().json(ServerToClientMessage::InitializeResult(Success {
                    id,
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result: InitializeResult {
                        capabilities: ServerCapabilities {
                            completions: None,
                            experimental: None,
                            logging: None,
                            prompts: None,
                            resources: None,
                            tools: None,
                        },
                        instructions: None,
                        protocol_version: MCP_PROTOCOL_VERSION.to_string(),
                        server_info: self.server_info.clone(),
                    },
                })),
            ),
            ClientToServerMessage::Ping(Request { id, .. }) => Ok(HttpResponse::Ok().json(
                ServerToClientMessage::Pong(Success {
                    id,
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result: Pong {},
                }),
            )),
        }
    }
}
