pub mod embedded_resource;
pub mod text_content;

use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::content_block::embedded_resource::EmbeddedResource;
use crate::mcp::jsonrpc::content_block::text_content::TextContent;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "resource")]
    EmbeddedResource(EmbeddedResource),
    #[serde(rename = "text")]
    TextContent(TextContent),
}
