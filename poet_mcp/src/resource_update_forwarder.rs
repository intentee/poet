use std::sync::Arc;

use tokio::select;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::jsonrpc_version::JSONRPC_VERSION;
use crate::notification_delivery::NotificationDelivery;
use crate::resources_updated_notification::ResourcesUpdatedNotification;
use crate::resources_updated_notification_params::ResourcesUpdatedNotificationParams;
use crate::server_to_client_notification::ServerToClientNotification;
use crate::session_notifier::SessionNotifier;

pub struct ResourceUpdateForwarder {
    pub cancellation_token: CancellationToken,
    pub resource_update_notifier: Arc<Notify>,
    pub session_notifier: SessionNotifier,
    pub uri: String,
}

impl ResourceUpdateForwarder {
    pub async fn forward(self) {
        loop {
            select! {
                biased;
                () = self.cancellation_token.cancelled() => break,
                () = self.resource_update_notifier.notified() => self.notify_session().await,
            }
        }
    }

    async fn notify_session(&self) {
        match self
            .session_notifier
            .notify(ServerToClientNotification::ResourcesUpdated(
                ResourcesUpdatedNotification {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                    params: ResourcesUpdatedNotificationParams {
                        uri: self.uri.clone(),
                    },
                },
            ))
            .await
        {
            NotificationDelivery::Delivered => {}
            NotificationDelivery::EventStreamClosed => self.cancellation_token.cancel(),
        }
    }
}
