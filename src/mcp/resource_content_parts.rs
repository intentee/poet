use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;

#[derive(Deserialize, Serialize)]
pub struct ResourceContentParts {
    pub parts: Vec<ResourceContent>,
    pub title: String,
    pub uri: String,
}
