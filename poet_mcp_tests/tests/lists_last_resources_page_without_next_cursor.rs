use actix_web::test::call_service;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn lists_last_resources_page_without_next_cursor() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession { session_id, .. } = InitializedSession::start(&mcp_service).await?;
    let response = read_json_response(
        call_service(
            &mcp_service,
            mcp_session_request(&session_id, &json!({ "id": 2, "jsonrpc": "2.0", "method": "resources/list", "params": { "cursor": STANDARD.encode(r#"{"offset":2,"per_page":2}"#) } })).to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(
        response,
        json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "resources": [{
                    "description": "Third document",
                    "name": "third",
                    "title": "third",
                    "uri": "memory://documents/third",
                }],
            },
        })
    );

    Ok(())
}
