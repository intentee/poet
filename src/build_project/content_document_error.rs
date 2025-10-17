use std::cmp::Ordering;
use std::fmt;

use crate::content_document_reference::ContentDocumentReference;

pub struct ContentDocumentError {
    pub content_document_reference: ContentDocumentReference,
    pub err: anyhow::Error,
}

impl fmt::Display for ContentDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.content_document_reference.basename())?;

        for cause in self.err.chain() {
            writeln!(formatter, "- {cause}")?;
        }

        Ok(())
    }
}

impl Eq for ContentDocumentError {}

impl Ord for ContentDocumentError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.content_document_reference
            .cmp(&other.content_document_reference)
    }
}

impl PartialEq for ContentDocumentError {
    fn eq(&self, other: &Self) -> bool {
        self.content_document_reference == other.content_document_reference
    }
}

impl PartialOrd for ContentDocumentError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
