use serde::Deserialize;
use serde::Serialize;

use crate::content_document_basename::ContentDocumentBasename;

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionPlacement {
    #[serde(default)]
    pub after: Option<ContentDocumentBasename>,
    pub name: String,
    #[serde(default)]
    pub parent: Option<ContentDocumentBasename>,
}
