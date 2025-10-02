use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::resources_updated::ResourcesUpdated;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ServerToClientNotification {
    #[serde(rename = "notifications/resources/updated")]
    ResourcesUpdated(ResourcesUpdated),
}
