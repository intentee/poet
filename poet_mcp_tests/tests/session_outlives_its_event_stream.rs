use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::json;

#[actix_web::test]
async fn session_outlives_its_event_stream() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession {
        event_stream,
        session_id,
        ..
    } = InitializedSession::start(&mcp_service).await?;

    drop(event_stream);

    let response = call_service(
        &mcp_service,
        mcp_session_request(
            &session_id,
            &json!({ "id": 2, "jsonrpc": "2.0", "method": "tools/list" }),
        )
        .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
