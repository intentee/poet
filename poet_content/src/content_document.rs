use markdown::mdast::Node;

use crate::content_document_reference::ContentDocumentReference;

pub struct ContentDocument {
    pub mdast: Node,
    pub reference: ContentDocumentReference,
}
