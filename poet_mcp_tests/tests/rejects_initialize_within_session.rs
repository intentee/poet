use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialize_message::initialize_message;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::json;

#[actix_web::test]
async fn rejects_initialize_within_session() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession { session_id, .. } = InitializedSession::start(&mcp_service).await?;
    let response = call_service(
        &mcp_service,
        mcp_session_request(&session_id, &initialize_message(&json!({}))).to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
