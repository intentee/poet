use markdown::mdast::Node;

use crate::markdown_document_reference::MarkdownDocumentReference;

pub struct MarkdownDocument {
    pub mdast: Node,
    pub reference: MarkdownDocumentReference,
}
