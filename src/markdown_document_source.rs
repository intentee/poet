use crate::filesystem::file_entry::FileEntry;
use crate::markdown_document_reference::MarkdownDocumentReference;

pub struct MarkdownDocumentSource {
    pub file_entry: FileEntry,
    pub reference: MarkdownDocumentReference,
}

impl MarkdownDocumentSource {
    pub fn relative_path(&self) -> String {
        self.file_entry.relative_path.display().to_string()
    }
}
