use std::collections::HashMap;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::front_matter::FrontMatter;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone)]
pub struct RhaiComponentContext {
    pub asset_manager: AssetManager,
    pub front_matter: FrontMatter,
    pub is_watching: bool,
    pub markdown_basename_by_id: Arc<HashMap<String, String>>,
    pub markdown_document_by_basename: Arc<HashMap<String, MarkdownDocumentReference>>,
}

impl RhaiComponentContext {
    pub fn get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    pub fn get_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }

    pub fn get_is_watching(&mut self) -> bool {
        self.is_watching
    }

    pub fn link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        let basename = if path.starts_with("#") {
            if let Some(basename) = self
                .markdown_basename_by_id
                .get(match path.strip_prefix('#') {
                    Some(id) => id,
                    None => return Err("Unable to strip prefix from document id".into()),
                })
            {
                basename
            } else {
                return Err(format!("Documetn with id does not exist: {path}").into());
            }
        } else {
            path
        };

        if let Some(reference) = self.markdown_document_by_basename.get(basename) {
            Ok(format!("/{}", reference.target_file_relative_path().display()).to_string())
        } else {
            Err(format!("Document does not exist: {path}").into())
        }
    }
}

impl CustomType for RhaiComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiComponentContext")
            .with_get("assets", Self::get_assets)
            .with_get("front_matter", Self::get_front_matter)
            .with_get("is_watching", Self::get_is_watching)
            .with_fn("link_to", Self::link_to);
    }
}
