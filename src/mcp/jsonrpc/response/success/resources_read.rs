use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlobResourceContent {
    pub blob: String,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextResourceContent {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ResourceContent {
    Blob(BlobResourceContent),
    Text(TextResourceContent),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesRead {
    pub contents: Vec<ResourceContent>,
}
