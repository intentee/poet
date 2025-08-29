use crate::front_matter::collection::Collection;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone, Debug)]
pub struct MarkdownDocumentInCollection {
    pub collection: Collection,
    pub reference: MarkdownDocumentReference,
}
