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

    pub fn is_subscribed_to_resource(&self, resource_uri: &str) -> bool {
        self.resource_subscriptions.contains_key(resource_uri)
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
