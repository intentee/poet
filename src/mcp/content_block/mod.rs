pub mod embedded_resource;
pub mod resource_link;
pub mod text_content;

use serde::Deserialize;
use serde::Serialize;

use crate::mcp::content_block::embedded_resource::EmbeddedResource;
use crate::mcp::content_block::resource_link::ResourceLink;
use crate::mcp::content_block::text_content::TextContent;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "resource")]
    EmbeddedResource(EmbeddedResource),
    #[serde(rename = "resource_link")]
    ResourceLink(ResourceLink),
    #[serde(rename = "text")]
    TextContent(TextContent),
}
