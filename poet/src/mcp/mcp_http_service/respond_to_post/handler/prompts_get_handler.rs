use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;

use crate::holder::Holder as _;
use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::prompts_get::PromptsGet;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;

pub struct PromptsGetHandler {
    pub prompt_controller_collection_holder: PromptControllerCollectionHolder,
}

#[async_trait]
impl Handler for PromptsGetHandler {
    type Request = PromptsGet;
    type Session = Session;

    async fn handle(
        self,
        request: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = match self
            .prompt_controller_collection_holder
            .get()
            .await {
            Some(prompt_controller_collection) => {
                match prompt_controller_collection.0.get(&request.params.name) {
                    Some(prompt_controller) => {
                        ServerToClientResponse::PromptsGet(Success {
                            id: request.id.clone(),
                            jsonrpc: JSONRPC_VERSION.to_string(),
                            result: prompt_controller
                                .respond_to(request)
                                .await
                                .map_err(ErrorInternalServerError)?,
                        })
                    }
                    None => {
                        ServerToClientResponse::Error(Error::invalid_prompt_name(
                            request.id,
                            request.params.name
                        ))
                    }
                }
            }
            None => {
                ServerToClientResponse::Error(Error::request_internal(
                    request.id,
                    "Prompt controller collection is not ready. The server is not ready yet or is still starting.".to_string(),
                ))
            }
        };

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
