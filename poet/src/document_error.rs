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
