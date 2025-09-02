use std::cmp::Ordering;

use log::error;

use crate::markdown_document_reference::MarkdownDocumentReference;

pub struct DocumentError {
    pub err: anyhow::Error,
    pub markdown_document_reference: MarkdownDocumentReference,
}

impl DocumentError {
    pub fn render(&self) {
        let mut error_chain = String::new();

        for cause in self.err.chain() {
            error_chain.push_str(&format!("- {cause}\n"));
        }

        error!(
            "{}:\n{error_chain}",
            self.markdown_document_reference.basename(),
        );
    }
}

impl Eq for DocumentError {}

impl Ord for DocumentError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.markdown_document_reference
            .cmp(&other.markdown_document_reference)
    }
}

impl PartialEq for DocumentError {
    fn eq(&self, other: &Self) -> bool {
        self.markdown_document_reference == other.markdown_document_reference
    }
}

impl PartialOrd for DocumentError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
