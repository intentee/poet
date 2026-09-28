use tokio::sync::mpsc::Receiver;

use crate::server_to_client_notification::ServerToClientNotification;
use crate::session::Session;

pub struct SessionWithNotificationsReceiver {
    pub notification_rx: Receiver<ServerToClientNotification>,
    pub session: Session,
}
