use std::sync::Arc;

use actix_web::rt;
use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio_util::sync::CancellationToken;

use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::log_level::LogLevel;

#[derive(Clone)]
pub struct Session {
    log_level: LogLevel,
    notification_tx: Sender<ServerToClientNotification>,
    session_id: String,
    subscriptions: Arc<DashMap<String, CancellationToken>>,
}

impl Session {
    pub fn new(notification_tx: Sender<ServerToClientNotification>, session_id: String) -> Self {
        Self {
            log_level: LogLevel::Info,
            notification_tx,
            session_id,
            subscriptions: Default::default(),
        }
    }

    pub fn id(&self) -> String {
        self.session_id.clone()
    }

    pub async fn notify(
        &self,
        notification: ServerToClientNotification,
    ) -> Result<(), SendError<ServerToClientNotification>> {
        self.notification_tx.send(notification).await
    }

    pub fn subscribe_to(&self, uri: &str) -> Result<CancellationToken> {
        if self.subscriptions.contains_key(uri) {
            return Err(anyhow!("Session is already subscribed to '{uri}'"));
        }

        let cancellation_token = CancellationToken::new();
        let subscriptions = self.subscriptions.clone();

        subscriptions.insert(uri.to_string(), cancellation_token.clone());

        let cancellation_token_clone = cancellation_token.clone();
        let uri_clone: String = uri.to_string();

        rt::spawn(async move {
            cancellation_token_clone.cancelled().await;
            subscriptions.remove(&uri_clone);
        });

        Ok(cancellation_token)
    }

    pub async fn terminate(self) {
        for ref_multi in self.subscriptions.iter() {
            ref_multi.value().cancel();
        }
    }

    pub fn with_log_level(self, log_level: LogLevel) -> Self {
        Self {
            log_level,
            notification_tx: self.notification_tx,
            session_id: self.session_id,
            subscriptions: self.subscriptions,
        }
    }
}
