use std::sync::Arc;

use actix_web::Result;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorInternalServerError;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use uuid::Uuid;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::session::Session;
use crate::mcp::session_storage::SessionStorage;
use crate::mcp::session_with_notifications_receiver::SessionWithNotificationsReceiver;

fn generate_session_id() -> String {
    format!("poet-{}", Uuid::new_v4())
}

#[derive(Clone, Default)]
pub struct SessionManager {
    pub session_storage: Arc<SessionStorage>,
}

impl SessionManager {
    pub async fn broadcast(
        &self,
        notification: ServerToClientNotification,
    ) -> Result<(), SendError<ServerToClientNotification>> {
        for entry in &self.session_storage.sessions {
            let session = entry.value();

            session.notify(notification.clone()).await?;
        }

        Ok(())
    }

    pub async fn restore_session(&self, req: &ServiceRequest) -> Result<Option<Session>> {
        match req.headers().get(MCP_HEADER_SESSION) {
            Some(session_id) => {
                self.session_storage
                    .read(session_id.to_str().map_err(ErrorInternalServerError)?)
                    .await
            }
            None => Ok(None),
        }
    }

    pub async fn start_new_session(&self) -> Result<SessionWithNotificationsReceiver> {
        let (notification_tx, notification_rx) = mpsc::channel(30);
        let session = Session::new(notification_tx, generate_session_id());

        self.session_storage
            .store_new_session(session.clone())
            .await?;

        Ok(SessionWithNotificationsReceiver {
            notification_rx,
            session,
        })
    }

    #[inline]
    pub async fn update_session(&self, session: Session) -> Result<()> {
        self.session_storage.update_session(session).await
    }

    #[inline]
    pub async fn terminate_session(&self, session: Session) -> Result<()> {
        self.session_storage.terminate_session(session).await
    }
}

#[cfg(test)]
mod tests {
    use actix_web::error::ErrorInternalServerError;
    use actix_web::test::TestRequest;

    use super::*;
    use crate::mcp::jsonrpc::JSONRPC_VERSION;
    use crate::mcp::jsonrpc::notification::message::Message;
    use crate::mcp::jsonrpc::notification::message::MessageParams;
    use crate::mcp::log_level::LogLevel;

    fn notification() -> ServerToClientNotification {
        ServerToClientNotification::Message(Message {
            jsonrpc: JSONRPC_VERSION.to_string(),
            params: MessageParams {
                data: "broadcast".to_string(),
                level: LogLevel::Info,
            },
        })
    }

    #[actix_web::test]
    async fn start_new_session_generates_unique_prefixed_ids() -> Result<()> {
        let manager = SessionManager::default();

        let first = manager.start_new_session().await?;
        let second = manager.start_new_session().await?;

        assert!(first.session.id().starts_with("poet-"));
        assert_ne!(first.session.id(), second.session.id());

        Ok(())
    }

    #[actix_web::test]
    async fn restore_session_returns_none_without_header() -> Result<()> {
        let manager = SessionManager::default();
        let request = TestRequest::default().to_srv_request();

        assert!(manager.restore_session(&request).await?.is_none());

        Ok(())
    }

    #[actix_web::test]
    async fn restore_session_returns_none_for_unknown_id() -> Result<()> {
        let manager = SessionManager::default();
        let request = TestRequest::default()
            .insert_header((MCP_HEADER_SESSION, "poet-unknown"))
            .to_srv_request();

        assert!(manager.restore_session(&request).await?.is_none());

        Ok(())
    }

    #[actix_web::test]
    async fn restore_session_returns_stored_session_for_known_id() -> Result<()> {
        let manager = SessionManager::default();
        let started = manager.start_new_session().await?;
        let session_id = started.session.id();
        let request = TestRequest::default()
            .insert_header((MCP_HEADER_SESSION, session_id.as_str()))
            .to_srv_request();

        let Some(restored) = manager.restore_session(&request).await? else {
            panic!("expected a stored session");
        };

        assert_eq!(restored.id(), session_id);

        Ok(())
    }

    #[actix_web::test]
    async fn broadcast_delivers_notification_to_every_session() -> Result<()> {
        let manager = SessionManager::default();
        let mut first = manager.start_new_session().await?;
        let mut second = manager.start_new_session().await?;

        manager
            .broadcast(notification())
            .await
            .map_err(ErrorInternalServerError)?;

        assert!(first.notification_rx.try_recv().is_ok());
        assert!(second.notification_rx.try_recv().is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn terminate_session_removes_it_from_storage() -> Result<()> {
        let manager = SessionManager::default();
        let started = manager.start_new_session().await?;
        let session_id = started.session.id();

        manager.terminate_session(started.session.clone()).await?;

        let request = TestRequest::default()
            .insert_header((MCP_HEADER_SESSION, session_id.as_str()))
            .to_srv_request();

        assert!(manager.restore_session(&request).await?.is_none());

        Ok(())
    }
}
