use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ContentDocumentBasename(pub String);

impl ContentDocumentBasename {
    pub fn get_collection_name(&self) -> String {
        self.0.split("/").map(String::from).collect::<Vec<String>>()[0].clone()
    }

    pub fn is_child_from(&self, parent: String) -> bool {
        self.0.starts_with(&format!("{}/", parent)) && !self.0.ends_with("index")
    }
}

impl Display for ContentDocumentBasename {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<PathBuf> for ContentDocumentBasename {
    fn from(basename_path: PathBuf) -> Self {
        Self(basename_path.display().to_string())
    }
}

impl From<String> for ContentDocumentBasename {
    fn from(basename: String) -> Self {
        Self(basename)
    }
}
