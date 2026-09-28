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
