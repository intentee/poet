use std::fmt;

use dashmap::DashMap;
use itertools::Itertools as _;

use crate::build_project::document_error::DocumentError;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Default)]
pub struct DocumentErrorCollection {
    errors: DashMap<MarkdownDocumentReference, Vec<DocumentError>>,
}

impl DocumentErrorCollection {
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn register_error(
        &self,
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
}

impl fmt::Display for DocumentErrorCollection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "Multiple errors occurred ({} total):",
            self.errors.len()
        )?;

        for errors in self
            .errors
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.key().basename(), &b.key().basename()))
        {
            for error in errors.value() {
                writeln!(formatter, "{error}")?;
            }
        }

        Ok(())
    }
}
