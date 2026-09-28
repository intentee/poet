use serde::Deserialize;
use serde::Serialize;

use crate::resources_updated_notification_params::ResourcesUpdatedNotificationParams;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesUpdatedNotification {
    pub jsonrpc: String,
    pub params: ResourcesUpdatedNotificationParams,
}
