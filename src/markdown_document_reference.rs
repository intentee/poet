use std::path::PathBuf;

use rhai::CustomType;
use rhai::TypeBuilder;

use crate::front_matter::FrontMatter;

#[derive(Clone, Debug)]
pub struct MarkdownDocumentReference {
    pub basename: String,
    pub basename_path: PathBuf,
    pub front_matter: FrontMatter,
}

impl MarkdownDocumentReference {
    pub fn get_basename(&mut self) -> String {
        self.basename.clone()
    }

    pub fn get_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }

    pub fn target_file_relative_path(&self) -> PathBuf {
        self.basename_path.with_extension("html")
    }
}

impl CustomType for MarkdownDocumentReference {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("MarkdownDocumentReference")
            .with_get("basename", Self::get_basename)
            .with_get("front_matter", Self::get_front_matter);
    }
}
