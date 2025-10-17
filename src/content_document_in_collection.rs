use crate::content_document_reference::ContentDocumentReference;
use crate::front_matter::collection_placement::CollectionPlacement;

#[derive(Clone, Debug)]
pub struct ContentDocumentInCollection {
    pub collection_placement: CollectionPlacement,
    pub reference: ContentDocumentReference,
}
