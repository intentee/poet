use actix_web::body::BoxBody;
use actix_web::body::to_bytes;
use actix_web::dev::ServiceResponse;
use serde_json::Value;
use serde_json::from_slice;

use crate::poet_mcp_tests_error::PoetMcpTestsError;

pub async fn read_json_response(
    service_response: ServiceResponse<BoxBody>,
) -> Result<Value, PoetMcpTestsError> {
    let response_body = to_bytes(service_response.into_body())
        .await
        .map_err(PoetMcpTestsError::ReadResponseBody)?;

    from_slice(&response_body).map_err(PoetMcpTestsError::ParseResponseBody)
}
