use std::sync::Arc;

use actix_web::http::header::HeaderMap;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::mcp_error::McpError;
use crate::mcp_header_session::MCP_HEADER_SESSION;
use crate::request_session::RequestSession;
use crate::server_to_client_notification::ServerToClientNotification;
use crate::session::Session;
use crate::session_notifier::SessionNotifier;
use crate::session_storage::SessionStorage;
use crate::session_subscriptions::SessionSubscriptions;
use crate::session_with_notifications_receiver::SessionWithNotificationsReceiver;

const NOTIFICATION_BUFFER_SIZE: usize = 30;

#[derive(Clone, Default)]
pub struct SessionManager {
    pub session_storage: Arc<SessionStorage>,
}

impl SessionManager {
    pub async fn broadcast(&self, notification: ServerToClientNotification) {
        let session_notifiers: Vec<SessionNotifier> = self
            .session_storage
            .sessions
            .iter()
            .map(|stored_session| stored_session.value().notifier.clone())
            .collect();

        for session_notifier in session_notifiers {
            session_notifier.notify(notification.clone()).await;
        }
    }

    pub fn request_session(&self, headers: &HeaderMap) -> Result<RequestSession, McpError> {
        let Some(session_header) = headers.get(MCP_HEADER_SESSION) else {
            return Ok(RequestSession::Absent);
        };
        let session_id = session_header
            .to_str()
            .map_err(|source| McpError::InvalidSessionHeader { source })?;

        self.session_storage
            .read(session_id)
            .map(RequestSession::Established)
            .ok_or_else(|| McpError::SessionNotFound {
                session_id: session_id.to_owned(),
            })
    }

    #[must_use]
    pub fn start_new_session(&self) -> SessionWithNotificationsReceiver {
        let (notification_tx, notification_rx) = mpsc::channel(NOTIFICATION_BUFFER_SIZE);
        let session = Session {
            id: format!("poet-{}", Uuid::new_v4()),
            notifier: SessionNotifier::new(notification_tx),
            subscriptions: SessionSubscriptions::default(),
        };

        self.session_storage.store(session.clone());

        SessionWithNotificationsReceiver {
            notification_rx,
            session,
        }
    }

    pub fn terminate_session(&self, session_id: &str) {
        self.session_storage.terminate(session_id);
    }

    pub fn update_session(&self, session: Session) {
        self.session_storage.store(session);
    }
}
