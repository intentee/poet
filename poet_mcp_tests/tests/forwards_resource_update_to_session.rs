use std::sync::Arc;

use poet_mcp::resource_update_forwarder::ResourceUpdateForwarder;
use poet_mcp::server_to_client_notification::ServerToClientNotification;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::session_with_notifications_receiver::SessionWithNotificationsReceiver;
use tokio::join;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

#[actix_web::test]
async fn forwards_resource_update_to_session() {
    let SessionWithNotificationsReceiver {
        mut notification_rx,
        session,
    } = SessionManager::default().start_new_session();
    let cancellation_token = CancellationToken::new();
    let resource_update_notifier: Arc<Notify> = Arc::default();

    resource_update_notifier.notify_one();

    let ((), notification) = join!(
        ResourceUpdateForwarder {
            cancellation_token: cancellation_token.clone(),
            resource_update_notifier,
            session_notifier: session.notifier,
            uri: "memory://documents/first".to_owned(),
        }
        .forward(),
        async {
            let notification = notification_rx.recv().await;

            cancellation_token.cancel();

            notification
        }
    );

    assert!(matches!(
        notification,
        Some(ServerToClientNotification::ResourcesUpdated(_))
    ));
}
