use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::message::Message;
use crate::mcp::jsonrpc::notification::resources_list_changed::ResourcesListChanged;
use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdated;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ServerToClientNotification {
    #[serde(rename = "notifications/message")]
    Message(Message),
    #[serde(rename = "notifications/resources/list_changed")]
    ResourcesListChanged(ResourcesListChanged),
    #[serde(rename = "notifications/resources/updated")]
    ResourcesUpdated(ResourcesUpdated),
}
