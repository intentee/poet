use std::fmt;
use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AuthorBasename(pub String);

impl fmt::Display for AuthorBasename {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<&Path> for AuthorBasename {
    fn from(basename_path: &Path) -> Self {
        Self(basename_path.display().to_string())
    }
}
