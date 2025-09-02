use std::collections::BTreeMap;

use crate::build_project::document_error::DocumentError;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Default)]
pub struct DocumentErrorCollection {
    errors: BTreeMap<MarkdownDocumentReference, Vec<DocumentError>>,
}

impl DocumentErrorCollection {
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn register_error(
        &mut self,
        err: anyhow::Error,
        markdown_document_reference: MarkdownDocumentReference,
    ) {
        self.errors
            .entry(markdown_document_reference.clone())
            .or_default()
            .push(DocumentError {
                err,
                markdown_document_reference,
            });
    }

    pub fn render(&self) {
        for errors in self.errors.values() {
            for error in errors {
                error.render();
            }
        }
    }
}
