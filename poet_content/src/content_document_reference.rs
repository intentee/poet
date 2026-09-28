use std::path::PathBuf;
use std::sync::Arc;

use rhai::CustomType;
use rhai::TypeBuilder;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::generated_page_file_name::GENERATED_PAGE_FILE_NAME;

const INDEX_DOCUMENT_NAME: &str = "index";

#[derive(Clone, Debug)]
pub struct ContentDocumentReference {
    pub basename_path: PathBuf,
    pub front_matter: Arc<ContentDocumentFrontMatter>,
    pub generated_page_base_path: String,
}

impl ContentDocumentReference {
    #[must_use]
    pub fn basename(&self) -> ContentDocumentBasename {
        ContentDocumentBasename::from(self.basename_path.as_path())
    }

    #[must_use]
    pub fn basename_last_stem(&self) -> String {
        self.page_directory()
            .file_name()
            .map_or_else(String::new, |file_name| {
                file_name.to_string_lossy().into_owned()
            })
    }

    #[must_use]
    pub fn canonical_link(&self) -> String {
        format!("{}{}", self.generated_page_base_path, self.link_stem())
    }

    #[must_use]
    pub fn target_file_relative_path(&self) -> PathBuf {
        self.page_directory().join(GENERATED_PAGE_FILE_NAME)
    }

    fn link_stem(&self) -> String {
        let page_directory = self.page_directory();

        if page_directory.as_os_str().is_empty() {
            String::new()
        } else {
            format!("{}/", page_directory.display())
        }
    }

    fn page_directory(&self) -> PathBuf {
        let mut page_directory = self.basename_path.clone();

        if page_directory.ends_with(INDEX_DOCUMENT_NAME) {
            page_directory.pop();
        }

        page_directory
    }

    fn rhai_basename(&mut self) -> String {
        self.basename().to_string()
    }

    fn rhai_basename_last_stem(&mut self) -> String {
        self.basename_last_stem()
    }

    fn rhai_canonical_link(&mut self) -> String {
        self.canonical_link()
    }

    fn rhai_front_matter(&mut self) -> ContentDocumentFrontMatter {
        self.front_matter.as_ref().clone()
    }
}

impl CustomType for ContentDocumentReference {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentReference")
            .with_get("basename", Self::rhai_basename)
            .with_get("basename_last_stem", Self::rhai_basename_last_stem)
            .with_get("canonical_link", Self::rhai_canonical_link)
            .with_get("front_matter", Self::rhai_front_matter);
    }
}
