use std::cmp::Ordering;
use std::fmt;

pub struct DocumentError {
    pub basename: String,
    pub err: anyhow::Error,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.basename)?;

        for cause in self.err.chain() {
            writeln!(formatter, "- {cause}")?;
        }

        Ok(())
    }
}

impl Eq for DocumentError {}

impl Ord for DocumentError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.basename.cmp(&other.basename)
    }
}

impl PartialEq for DocumentError {
    fn eq(&self, other: &Self) -> bool {
        self.basename == other.basename
    }
}

impl PartialOrd for DocumentError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn display_lists_basename_then_full_error_chain() {
        let document_error = DocumentError {
            basename: "guide".to_string(),
            err: anyhow!("root cause").context("outer context"),
        };

        assert_eq!(
            format!("{document_error}"),
            "guide:\n- outer context\n- root cause\n"
        );
    }

    #[test]
    fn orders_by_basename() {
        let alpha = DocumentError {
            basename: "alpha".to_string(),
            err: anyhow!("first"),
        };
        let beta = DocumentError {
            basename: "beta".to_string(),
            err: anyhow!("second"),
        };

        assert!(alpha < beta);
    }

    #[test]
    fn equality_is_decided_by_basename() {
        let one = DocumentError {
            basename: "guide".to_string(),
            err: anyhow!("first"),
        };
        let another = DocumentError {
            basename: "guide".to_string(),
            err: anyhow!("second"),
        };

        assert!(one == another);
    }
}
