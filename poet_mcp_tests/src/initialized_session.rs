use actix_http::Request;
use actix_web::Error;
use actix_web::body::BoxBody;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::test::call_service;
use poet_mcp::mcp_header_session::MCP_HEADER_SESSION;
use serde_json::Value;
use serde_json::json;

use crate::event_stream::EventStream;
use crate::initialize_message::initialize_message;
use crate::mcp_request::mcp_request;
use crate::poet_mcp_tests_error::PoetMcpTestsError;

pub struct InitializedSession {
    pub event_stream: EventStream,
    pub initialize_result: Value,
    pub session_id: String,
}

impl InitializedSession {
    pub async fn start<TService>(mcp_service: &TService) -> Result<Self, PoetMcpTestsError>
    where
        TService: Service<Request, Response = ServiceResponse<BoxBody>, Error = Error>,
    {
        let service_response = call_service(
            mcp_service,
            mcp_request(&initialize_message(&json!({}))).to_request(),
        )
        .await;
        let session_id = service_response
            .headers()
            .get(MCP_HEADER_SESSION)
            .ok_or(PoetMcpTestsError::MissingSessionHeader)?
            .to_str()
            .map_err(PoetMcpTestsError::InvalidSessionHeader)?
            .to_owned();
        let mut event_stream = EventStream {
            body: service_response.into_body(),
        };
        let initialize_result = event_stream.next_event().await?;

        Ok(Self {
            event_stream,
            initialize_result,
            session_id,
        })
    }
}
