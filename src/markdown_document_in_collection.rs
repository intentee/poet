use crate::front_matter::collection_placement::CollectionPlacement;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone, Debug)]
pub struct MarkdownDocumentInCollection {
    pub collection_placement: CollectionPlacement,
    pub reference: MarkdownDocumentReference,
}
