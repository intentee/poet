use std::collections::HashMap;
use std::sync::Arc;

use crate::author_collection::AuthorCollection;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_linker::ContentDocumentLinker;

#[derive(Clone)]
pub struct ContentSiteContext {
    pub available_authors: Arc<AuthorCollection>,
    pub content_document_collections_ranked: Arc<HashMap<String, ContentDocumentCollectionRanked>>,
    pub content_document_linker: ContentDocumentLinker,
    pub is_watching: bool,
}
