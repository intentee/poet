use poet_mcp::log_level::LogLevel;
use poet_mcp::message_notification_params::MessageNotificationParams;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::session_with_notifications_receiver::SessionWithNotificationsReceiver;

#[actix_web::test]
async fn drops_log_messages_below_default_info_level() {
    let SessionWithNotificationsReceiver {
        notification_rx,
        session,
    } = SessionManager::default().start_new_session();

    session
        .notifier
        .log_message(MessageNotificationParams {
            data: "details".to_owned(),
            level: LogLevel::Debug,
        })
        .await;

    assert!(notification_rx.is_empty());
}
