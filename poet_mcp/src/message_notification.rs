use serde::Deserialize;
use serde::Serialize;

use crate::message_notification_params::MessageNotificationParams;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MessageNotification {
    pub jsonrpc: String,
    pub params: MessageNotificationParams,
}
