use tokio::sync::mpsc::Sender;

use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::log_level::LogLevel;

#[derive(Clone)]
pub struct Session {
    pub log_level: LogLevel,
    pub notification_tx: Sender<ServerToClientNotification>,
    pub session_id: String,
}

impl Session {
    pub fn with_log_level(self, log_level: LogLevel) -> Self {
        Self {
            log_level,
            notification_tx: self.notification_tx,
            session_id: self.session_id,
        }
    }
}
