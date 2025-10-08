use markdown::mdast::Node;

use crate::filesystem::file_entry::FileEntry;
use crate::markdown_document_reference::MarkdownDocumentReference;

pub struct MarkdownDocumentSource {
    pub file_entry: FileEntry,
    pub mdast: Node,
    pub relative_path: String,
    pub reference: MarkdownDocumentReference,
}
