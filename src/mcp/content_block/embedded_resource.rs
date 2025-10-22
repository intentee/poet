use serde::Deserialize;
use serde::Serialize;

use crate::mcp::resource_content::ResourceContent;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbeddedResource {
    resource: ResourceContent,
}
