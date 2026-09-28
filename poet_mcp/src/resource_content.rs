use serde::Deserialize;
use serde::Serialize;

use crate::blob_resource_content::BlobResourceContent;
use crate::text_resource_content::TextResourceContent;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ResourceContent {
    Blob(BlobResourceContent),
    Text(TextResourceContent),
}
