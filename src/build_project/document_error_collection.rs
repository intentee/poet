use std::collections::BTreeMap;

use crate::build_project::document_error::DocumentError;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Default)]
pub struct DocumentErrorCollection {
    pub errors: BTreeMap<MarkdownDocumentReference, Vec<DocumentError>>,
}

impl DocumentErrorCollection {
    pub fn render(&self) {
        for errors in self.errors.values() {
            for error in errors {
                error.render();
            }
        }
    }
}
