use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BlogName(pub String);

impl BlogName {
    pub fn relative_blog_directory(&self) -> PathBuf {
        PathBuf::from("blogs").join(&self.0)
    }
}

impl Display for BlogName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<PathBuf> for BlogName {
    fn from(path: PathBuf) -> Self {
        Self(path.display().to_string())
    }
}

impl From<String> for BlogName {
    fn from(blog_name: String) -> Self {
        Self(blog_name)
    }
}
