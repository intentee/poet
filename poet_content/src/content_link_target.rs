use crate::content_document_basename::ContentDocumentBasename;

pub enum ContentLinkTarget {
    DocumentId(String),
    DocumentPath(ContentDocumentBasename),
}

impl From<&str> for ContentLinkTarget {
    fn from(link: &str) -> Self {
        link.strip_prefix('#').map_or_else(
            || Self::DocumentPath(ContentDocumentBasename(link.to_owned())),
            |document_id| Self::DocumentId(document_id.to_owned()),
        )
    }
}
