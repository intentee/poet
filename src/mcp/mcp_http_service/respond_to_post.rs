use actix_web::FromRequest as _;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::web::Json;
use async_trait::async_trait;
use mime::Mime;

use crate::jsonrpc::client_to_server_message::ClientToServerMessage;
use crate::jsonrpc::implementation::Implementation;
use crate::jsonrpc::request::Request;
use crate::jsonrpc::request::initialize::Initialize;
use crate::jsonrpc::response::error::Error;
use crate::jsonrpc::response::success::Success;
use crate::jsonrpc::response::success::initialize_result::InitializeResult;
use crate::jsonrpc::response::success::initialize_result::ServerCapabilities;
use crate::jsonrpc::server_to_client_message::ServerToClientMessage;
use crate::mcp::mcp_responder::McpResponder;

#[derive(Clone)]
pub struct RespondToPost {}

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
        let json: ClientToServerMessage =
            Json::<ClientToServerMessage>::from_request(&req, &mut payload)
                .await?
                .into_inner();
        println!("{json:?}");

        match json {
            ClientToServerMessage::Notification(_) => {
                Ok(HttpResponse::BadRequest().json(Error::invalid_request()))
            }
            ClientToServerMessage::Initialize(Request {
                id,
                jsonrpc,
                payload: Initialize { method, params },
            }) => {
                println!("INITIALIZE");

                Ok(
                    HttpResponse::Ok().json(ServerToClientMessage::InitializeResult(Success {
                        id,
                        jsonrpc,
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
                            protocol_version: "2025-06-18".to_string(),
                            server_info: Implementation {
                                name: "poet".to_string(),
                                title: Some("Poet".to_string()),
                                version: env!("CARGO_PKG_VERSION").to_string(),
                            },
                        },
                    })),
                )
            }
        }
    }
}
