use poet_mcp::session_manager::SessionManager;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[actix_web::test]
async fn terminated_session_is_forgotten_and_its_subscriptions_cancelled()
-> Result<(), PoetMcpTestsError> {
    let session_manager = SessionManager::default();
    let session = session_manager.start_new_session().session;
    let cancellation_token = session
        .subscriptions
        .subscribe("memory://documents/first")?;

    session_manager.terminate_session(&session.id);

    assert!(cancellation_token.is_cancelled());
    assert!(session_manager.session_storage.read(&session.id).is_none());

    Ok(())
}
