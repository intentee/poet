use serde::Deserialize;
use serde::Serialize;

use crate::message_notification::MessageNotification;
use crate::resources_list_changed_notification::ResourcesListChangedNotification;
use crate::resources_updated_notification::ResourcesUpdatedNotification;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ServerToClientNotification {
    #[serde(rename = "notifications/message")]
    Message(MessageNotification),
    #[serde(rename = "notifications/resources/list_changed")]
    ResourcesListChanged(ResourcesListChangedNotification),
    #[serde(rename = "notifications/resources/updated")]
    ResourcesUpdated(ResourcesUpdatedNotification),
}
