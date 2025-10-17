use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone, Debug)]
pub struct SearchIndexFoundDocument {
    pub content_document_reference: ContentDocumentReference,
}
