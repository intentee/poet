use serde::Deserialize;
use serde::Serialize;

use crate::mcp::resource_content::ResourceContent;

#[derive(Deserialize, Serialize)]
pub struct ResourceContentParts {
    pub parts: Vec<ResourceContent>,
    pub title: String,
    pub uri: String,
}
