use poet_mcp::jsonrpc_version::JSONRPC_VERSION;
use poet_mcp::resources_list_changed_notification::ResourcesListChangedNotification;
use poet_mcp::server_to_client_notification::ServerToClientNotification;
use poet_mcp::session_manager::SessionManager;

#[actix_web::test]
async fn broadcast_skips_sessions_without_open_event_stream() {
    let session_manager = SessionManager::default();
    let mut open_session = session_manager.start_new_session();

    drop(session_manager.start_new_session().notification_rx);

    session_manager
        .broadcast(ServerToClientNotification::ResourcesListChanged(
            ResourcesListChangedNotification {
                jsonrpc: JSONRPC_VERSION.to_owned(),
            },
        ))
        .await;

    assert!(matches!(
        open_session.notification_rx.recv().await,
        Some(ServerToClientNotification::ResourcesListChanged(_))
    ));
}
