use serde::Deserialize;

use crate::content_document_basename::ContentDocumentBasename;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionPlacement {
    #[serde(default)]
    pub after: Option<ContentDocumentBasename>,
    pub name: String,
    #[serde(default)]
    pub parent: Option<ContentDocumentBasename>,
}
