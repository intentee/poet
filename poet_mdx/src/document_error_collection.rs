use std::error::Error;
use std::fmt;

use dashmap::DashMap;

use crate::document_error::DocumentError;

#[derive(Debug)]
pub struct DocumentErrorCollection<TError> {
    document_errors: DashMap<String, Vec<DocumentError<TError>>>,
}

impl<TError> DocumentErrorCollection<TError> {
    #[must_use]
    pub fn into_document_errors(self) -> Vec<DocumentError<TError>> {
        let mut document_errors: Vec<DocumentError<TError>> = self
            .document_errors
            .into_iter()
            .flat_map(|(_, basename_errors)| basename_errors)
            .collect();

        document_errors.sort_by(|first_document_error, second_document_error| {
            first_document_error
                .basename
                .cmp(&second_document_error.basename)
        });

        document_errors
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.document_errors.is_empty()
    }

    pub fn register_error(&self, basename: String, error: TError) {
        self.document_errors
            .entry(basename.clone())
            .or_default()
            .push(DocumentError { basename, error });
    }
}

impl<TError> Default for DocumentErrorCollection<TError> {
    fn default() -> Self {
        Self {
            document_errors: DashMap::new(),
        }
    }
}

impl<TError: Error + 'static> fmt::Display for DocumentErrorCollection<TError> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut basenames: Vec<String> = self
            .document_errors
            .iter()
            .map(|document_errors| document_errors.key().clone())
            .collect();

        basenames.sort();

        writeln!(
            formatter,
            "Errors occurred in {} documents:",
            basenames.len()
        )
        .and_then(|()| {
            basenames
                .iter()
                .filter_map(|basename| self.document_errors.get(basename))
                .try_for_each(|document_errors| {
                    document_errors
                        .value()
                        .iter()
                        .try_for_each(|document_error| writeln!(formatter, "{document_error}"))
                })
        })
    }
}
