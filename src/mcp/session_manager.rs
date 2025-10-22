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

#[derive(Clone)]
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
