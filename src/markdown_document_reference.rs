use std::path::PathBuf;

use crate::front_matter::FrontMatter;

#[derive(Clone)]
pub struct MarkdownDocumentReference {
    pub basename: String,
    pub basename_path: PathBuf,
    pub front_matter: FrontMatter,
}

impl MarkdownDocumentReference {
    pub fn target_file_relative_path(&self) -> PathBuf {
        self.basename_path.with_extension("html")
    }
}
