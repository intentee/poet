use markdown::mdast::Node;
use poet_filesystem::file_entry::FileEntry;

use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone)]
pub struct ContentDocumentSource {
    pub file_entry: FileEntry,
    pub mdast: Node,
    pub reference: ContentDocumentReference,
}
