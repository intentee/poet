use std::sync::Arc;

use actix_web::rt;
use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio_util::sync::CancellationToken;

use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::notification::message::Message;
use crate::mcp::jsonrpc::notification::message::MessageParams;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::log_level::LogLevel;

#[derive(Clone)]
pub struct Session {
    log_level: LogLevel,
    notification_tx: Sender<ServerToClientNotification>,
    resource_subscriptions: Arc<DashMap<String, CancellationToken>>,
    session_id: String,
}

impl Session {
    pub fn new(notification_tx: Sender<ServerToClientNotification>, session_id: String) -> Self {
        Self {
            log_level: LogLevel::Info,
            notification_tx,
            resource_subscriptions: Default::default(),
            session_id,
        }
    }

    pub fn id(&self) -> String {
        self.session_id.clone()
    }

    pub async fn log(&self, message: Message) -> Result<(), SendError<ServerToClientNotification>> {
        if message.params.level >= self.log_level {
            self.notify(ServerToClientNotification::Message(message))
                .await
        } else {
            Ok(())
        }
    }

    pub async fn log_message(
        &self,
        params: MessageParams,
    ) -> Result<(), SendError<ServerToClientNotification>> {
        self.log(Message {
            jsonrpc: JSONRPC_VERSION.to_string(),
            params,
        })
        .await
    }

    pub async fn notify(
        &self,
        notification: ServerToClientNotification,
    ) -> Result<(), SendError<ServerToClientNotification>> {
        self.notification_tx.send(notification).await
    }

    pub async fn subscribe_to_resource(&self, uri: &str) -> Result<CancellationToken> {
        if self.resource_subscriptions.contains_key(uri) {
            let message = format!("You are already subscribed to '{uri}'");

            self.log_message(MessageParams {
                data: message.clone(),
                level: LogLevel::Error,
            })
            .await?;

            return Err(anyhow!("{message}"));
        }

        let cancellation_token = CancellationToken::new();
        let resource_subscriptions = self.resource_subscriptions.clone();

        resource_subscriptions.insert(uri.to_string(), cancellation_token.clone());

        let cancellation_token_clone = cancellation_token.clone();
        let uri_clone: String = uri.to_string();

        rt::spawn(async move {
            cancellation_token_clone.cancelled().await;
            resource_subscriptions.remove(&uri_clone);
        });

        Ok(cancellation_token)
    }

    pub fn subscribe_token(&self, uri: &str) -> Result<Option<CancellationToken>> {
        Ok(self
            .resource_subscriptions
            .get(uri)
            .map(|dashmap_ref| dashmap_ref.value().clone()))
    }

    pub async fn terminate(self) {
        for ref_multi in self.resource_subscriptions.iter() {
            ref_multi.value().cancel();
        }
    }

    pub fn with_log_level(self, log_level: LogLevel) -> Self {
        Self {
            log_level,
            notification_tx: self.notification_tx,
            resource_subscriptions: self.resource_subscriptions,
            session_id: self.session_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;

    fn message(level: LogLevel) -> Message {
        Message {
            jsonrpc: JSONRPC_VERSION.to_string(),
            params: MessageParams {
                data: "payload".to_string(),
                level,
            },
        }
    }

    #[tokio::test]
    async fn log_drops_messages_below_session_level() -> Result<()> {
        let (notification_tx, mut notification_rx) = mpsc::channel(4);
        let session = Session::new(notification_tx, "session-1".to_string());

        session.log(message(LogLevel::Debug)).await?;

        assert!(notification_rx.try_recv().is_err());

        Ok(())
    }

    #[tokio::test]
    async fn log_sends_messages_at_or_above_session_level() -> Result<()> {
        let (notification_tx, mut notification_rx) = mpsc::channel(4);
        let session = Session::new(notification_tx, "session-1".to_string());

        session.log(message(LogLevel::Error)).await?;

        assert!(notification_rx.try_recv().is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn with_log_level_raises_filtering_threshold() -> Result<()> {
        let (notification_tx, mut notification_rx) = mpsc::channel(4);
        let session =
            Session::new(notification_tx, "session-1".to_string()).with_log_level(LogLevel::Error);

        session.log(message(LogLevel::Info)).await?;

        assert!(notification_rx.try_recv().is_err());

        Ok(())
    }

    #[tokio::test]
    async fn log_message_wraps_params_into_notification() -> Result<()> {
        let (notification_tx, mut notification_rx) = mpsc::channel(4);
        let session = Session::new(notification_tx, "session-1".to_string());

        session
            .log_message(MessageParams {
                data: "details".to_string(),
                level: LogLevel::Warning,
            })
            .await?;

        let ServerToClientNotification::Message(received) = notification_rx.try_recv()? else {
            panic!("expected a logging message notification");
        };

        assert_eq!(received.params.data, "details");

        Ok(())
    }

    #[actix_web::test]
    async fn subscribe_registers_token_and_rejects_duplicates() -> Result<()> {
        let (notification_tx, _notification_rx) = mpsc::channel(4);
        let session = Session::new(notification_tx, "session-1".to_string());

        session
            .subscribe_to_resource("res://documents/guide")
            .await?;

        assert!(session.subscribe_token("res://documents/guide")?.is_some());
        assert!(
            session
                .subscribe_to_resource("res://documents/guide")
                .await
                .is_err()
        );

        Ok(())
    }

    #[actix_web::test]
    async fn terminate_cancels_subscription_tokens() -> Result<()> {
        let (notification_tx, _notification_rx) = mpsc::channel(4);
        let session = Session::new(notification_tx, "session-1".to_string());

        let cancellation_token = session
            .subscribe_to_resource("res://documents/guide")
            .await?;

        session.terminate().await;

        assert!(cancellation_token.is_cancelled());

        Ok(())
    }
}
