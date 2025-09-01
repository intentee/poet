use std::collections::HashMap;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::front_matter::FrontMatter;
use crate::markdown_document_collection::MarkdownDocumentCollection;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone)]
pub struct ComponentContext {
    pub asset_manager: AssetManager,
    pub collections: Arc<HashMap<String, MarkdownDocumentCollection>>,
    pub front_matter: FrontMatter,
    pub is_watching: bool,
    pub markdown_basename_by_id: Arc<HashMap<String, String>>,
    pub markdown_document_by_basename: Arc<HashMap<String, MarkdownDocumentReference>>,
}

impl ComponentContext {
    pub fn get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    pub fn link_to(&self, path: &str) -> Result<String, String> {
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
                return Err(format!("Document with id does not exist: {path}"));
            }
        } else {
            path
        };

        if let Some(reference) = self.markdown_document_by_basename.get(basename) {
            match reference.canonical_link() {
                Ok(canonical_link) => Ok(canonical_link),
                Err(err) => Err(format!(
                    "Unable to generate canonical link for {basename}: {err}"
                )),
            }
        } else {
            Err(format!("Document does not exist: {path}"))
        }
    }

    fn rhai_collection(
        &mut self,
        collection_name: &str,
    ) -> Result<MarkdownDocumentCollection, Box<EvalAltResult>> {
        if let Some(collection) = self.collections.get(collection_name) {
            Ok(collection.clone())
        } else {
            Err(format!("There are no documents in collection: '{collection_name}'").into())
        }
    }

    fn rhai_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }

    fn rhai_is_watching(&mut self) -> bool {
        self.is_watching
    }

    fn rhai_link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.link_to(path)?)
    }
}

impl CustomType for ComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ComponentContext")
            .with_get("assets", Self::get_assets)
            .with_get("front_matter", Self::rhai_front_matter)
            .with_get("is_watching", Self::rhai_is_watching)
            .with_fn("collection", Self::rhai_collection)
            .with_fn("link_to", Self::rhai_link_to);
    }
}
