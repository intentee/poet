use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

impl From<String> for BlogPostBasename {
    fn from(basename: String) -> Self {
        Self(basename)
    }
}
