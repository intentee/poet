use tokio::sync::mpsc::Receiver;

use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::session::Session;

pub struct SessionWithNotificationsReceiver {
    pub notification_rx: Receiver<ServerToClientNotification>,
    pub session: Session,
}
