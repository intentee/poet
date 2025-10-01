use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;

pub struct ResourceContentParts {
    pub parts: Vec<ResourceContent>,
}

impl From<ResourceContent> for ResourceContentParts {
    fn from(resource_content: ResourceContent) -> Self {
        Self {
            parts: vec![resource_content],
        }
    }
}
