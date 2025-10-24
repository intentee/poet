use serde::Deserialize;
use serde::Serialize;

use crate::mcp::resource_content::ResourceContent;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbeddedResource {
    resource: ResourceContent,
}
