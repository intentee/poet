use markdown::mdast::Node;

use crate::content_document_reference::ContentDocumentReference;
use crate::filesystem::file_entry::FileEntry;

#[derive(Clone)]
pub struct ContentDocumentSource {
    pub file_entry: FileEntry,
    pub mdast: Node,
    pub reference: ContentDocumentReference,
    pub relative_path: String,
}
