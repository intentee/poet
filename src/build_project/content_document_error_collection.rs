use std::fmt;

use dashmap::DashMap;
use itertools::Itertools as _;

use crate::build_project::content_document_error::ContentDocumentError;
use crate::content_document_reference::ContentDocumentReference;

#[derive(Default)]
pub struct ContentDocumentErrorCollection {
    errors: DashMap<ContentDocumentReference, Vec<ContentDocumentError>>,
}

impl ContentDocumentErrorCollection {
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn register_error(
        &self,
        content_document_reference: ContentDocumentReference,
        err: anyhow::Error,
    ) {
        self.errors
            .entry(content_document_reference.clone())
            .or_default()
            .push(ContentDocumentError {
                content_document_reference,
                err,
            });
    }
}

impl fmt::Display for ContentDocumentErrorCollection {
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
