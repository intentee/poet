use crate::collection_placement::CollectionPlacement;
use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone, Debug)]
pub struct ContentDocumentInCollection {
    pub collection_placement: CollectionPlacement,
    pub reference: ContentDocumentReference,
}
