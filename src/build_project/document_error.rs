use std::cmp::Ordering;
use std::fmt;

use crate::markdown_document_reference::MarkdownDocumentReference;

pub struct DocumentError {
    pub err: anyhow::Error,
    pub markdown_document_reference: MarkdownDocumentReference,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "{}:",
            self.markdown_document_reference.basename()
        )?;

        for cause in self.err.chain() {
            writeln!(formatter, "- {cause}")?;
        }

        Ok(())
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
