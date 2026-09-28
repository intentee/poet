use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn unsubscribing_allows_subscribing_again() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession { session_id, .. } = InitializedSession::start(&mcp_service).await?;
    let subscribe_message = json!({
        "id": 2,
        "jsonrpc": "2.0",
        "method": "resources/subscribe",
        "params": { "uri": "memory://documents/first" },
    });

    call_service(
        &mcp_service,
        mcp_session_request(&session_id, &subscribe_message).to_request(),
    )
    .await;

    let unsubscribe_response = read_json_response(
        call_service(
            &mcp_service,
            mcp_session_request(
                &session_id,
                &json!({
                    "id": 3,
                    "jsonrpc": "2.0",
                    "method": "resources/unsubscribe",
                    "params": { "uri": "memory://documents/first" },
                }),
            )
            .to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(
        unsubscribe_response,
        json!({ "id": 3, "jsonrpc": "2.0", "result": {} })
    );

    let resubscribe_response = read_json_response(
        call_service(
            &mcp_service,
            mcp_session_request(&session_id, &subscribe_message).to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(
        resubscribe_response,
        json!({ "id": 2, "jsonrpc": "2.0", "result": {} })
    );

    Ok(())
}
