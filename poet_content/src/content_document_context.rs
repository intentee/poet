use crate::author::Author;
use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone)]
pub struct ContentDocumentContext {
    pub authors: Vec<Author>,
    pub reference: ContentDocumentReference,
}
