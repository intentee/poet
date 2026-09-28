use serde::Deserialize;
use serde::Serialize;

use crate::embedded_resource::EmbeddedResource;
use crate::resource_link::ResourceLink;
use crate::text_content::TextContent;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "resource")]
    EmbeddedResource(EmbeddedResource),
    #[serde(rename = "resource_link")]
    ResourceLink(ResourceLink),
    #[serde(rename = "text")]
    TextContent(TextContent),
}

impl From<&str> for ContentBlock {
    fn from(text: &str) -> Self {
        Self::TextContent(text.into())
    }
}

impl From<String> for ContentBlock {
    fn from(text: String) -> Self {
        Self::TextContent(text.into())
    }
}
