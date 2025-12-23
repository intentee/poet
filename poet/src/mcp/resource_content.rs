use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlobResourceContent {
    pub blob: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextResourceContent {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ResourceContent {
    Blob(BlobResourceContent),
    Text(TextResourceContent),
}
