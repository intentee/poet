use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;

use crate::jsonrpc_version::JSONRPC_VERSION;
use crate::log_level::LogLevel;
use crate::message_notification::MessageNotification;
use crate::message_notification_params::MessageNotificationParams;
use crate::notification_delivery::NotificationDelivery;
use crate::server_to_client_notification::ServerToClientNotification;

#[derive(Clone)]
pub struct SessionNotifier {
    log_level: LogLevel,
    notification_tx: Sender<ServerToClientNotification>,
}

impl SessionNotifier {
    #[must_use]
    pub const fn new(notification_tx: Sender<ServerToClientNotification>) -> Self {
        Self {
            log_level: LogLevel::Info,
            notification_tx,
        }
    }

    pub async fn log_message(&self, params: MessageNotificationParams) {
        if params.level >= self.log_level {
            self.notify(ServerToClientNotification::Message(MessageNotification {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                params,
            }))
            .await;
        }
    }

    pub async fn notify(&self, notification: ServerToClientNotification) -> NotificationDelivery {
        match self.notification_tx.send(notification).await {
            Ok(()) => NotificationDelivery::Delivered,
            Err(SendError(_undelivered_notification)) => NotificationDelivery::EventStreamClosed,
        }
    }

    #[must_use]
    pub fn with_log_level(self, log_level: LogLevel) -> Self {
        Self { log_level, ..self }
    }
}
