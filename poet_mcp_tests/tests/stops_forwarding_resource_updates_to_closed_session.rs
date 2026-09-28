use std::sync::Arc;

use poet_mcp::resource_update_forwarder::ResourceUpdateForwarder;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::session_with_notifications_receiver::SessionWithNotificationsReceiver;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

#[actix_web::test]
async fn stops_forwarding_resource_updates_to_closed_session() {
    let SessionWithNotificationsReceiver {
        notification_rx,
        session,
    } = SessionManager::default().start_new_session();
    let cancellation_token = CancellationToken::new();
    let resource_update_notifier: Arc<Notify> = Arc::default();

    drop(notification_rx);
    resource_update_notifier.notify_one();

    ResourceUpdateForwarder {
        cancellation_token: cancellation_token.clone(),
        resource_update_notifier,
        session_notifier: session.notifier,
        uri: "memory://documents/first".to_owned(),
    }
    .forward()
    .await;

    assert!(cancellation_token.is_cancelled());
}
