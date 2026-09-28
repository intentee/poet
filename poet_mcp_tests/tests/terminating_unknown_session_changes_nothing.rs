use poet_mcp::session_manager::SessionManager;

#[actix_web::test]
async fn terminating_unknown_session_changes_nothing() {
    let session_manager = SessionManager::default();
    let session = session_manager.start_new_session().session;

    session_manager.terminate_session("poet-unknown");

    assert!(session_manager.session_storage.read(&session.id).is_some());
}
