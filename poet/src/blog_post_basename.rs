use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BlogPostBasename(pub String);

impl Display for BlogPostBasename {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<PathBuf> for BlogPostBasename {
    fn from(basename_path: PathBuf) -> Self {
        Self(basename_path.display().to_string())
    }
}
