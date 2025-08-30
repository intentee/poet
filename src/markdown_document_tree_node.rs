use std::collections::LinkedList;

use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Debug)]
pub struct MarkdownDocumentTreeNode {
    pub children: LinkedList<MarkdownDocumentTreeNode>,
    pub reference: MarkdownDocumentReference,
}
