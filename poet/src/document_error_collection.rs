use std::fmt;

use dashmap::DashMap;
use itertools::Itertools as _;

use crate::document_error::DocumentError;

#[derive(Default)]
pub struct DocumentErrorCollection {
    errors: DashMap<String, Vec<DocumentError>>,
}

impl DocumentErrorCollection {
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn register_error(&self, basename: String, err: anyhow::Error) {
        self.errors
            .entry(basename.clone())
            .or_default()
            .push(DocumentError { basename, err });
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
            .sorted_by(|a, b| Ord::cmp(&a.key(), &b.key()))
        {
            for error in errors.value() {
                writeln!(formatter, "{error:#}")?;
            }
        }

        Ok(())
    }
}
