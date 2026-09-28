use poet_mcp::session_manager::SessionManager;

#[actix_web::test]
async fn starts_sessions_with_unique_prefixed_ids() {
    let session_manager = SessionManager::default();
    let first_session = session_manager.start_new_session().session;
    let second_session = session_manager.start_new_session().session;

    assert!(first_session.id.starts_with("poet-"));
    assert_ne!(first_session.id, second_session.id);
}
