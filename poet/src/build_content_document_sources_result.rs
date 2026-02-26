use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::content_document::ContentDocument;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;

pub struct BuildContentDocumentSourcesResult {
    pub content_document_basename_by_id: HashMap<String, ContentDocumentBasename>,
    pub content_document_by_basename: HashMap<ContentDocumentBasename, ContentDocumentReference>,
    pub content_document_list: Vec<ContentDocument>,
    pub content_document_sources: BTreeMap<ContentDocumentBasename, ContentDocumentSource>,
}
