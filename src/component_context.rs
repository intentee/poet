use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::front_matter::FrontMatter;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::rhai_front_matter::RhaiFrontMatter;
use crate::rhai_markdown_document_collection::RhaiMarkdownDocumentCollection;
use crate::rhai_markdown_document_reference::RhaiMarkdownDocumentReference;

#[derive(Clone)]
pub struct ComponentContext {
    pub available_collections: Arc<HashSet<String>>,
    pub asset_manager: AssetManager,
    pub front_matter: FrontMatter,
    pub is_watching: bool,
    pub markdown_basename_by_id: Arc<HashMap<String, String>>,
    pub markdown_document_by_basename: Arc<HashMap<String, MarkdownDocumentReference>>,
    pub reference: MarkdownDocumentReference,
    pub rhai_markdown_document_collections: Arc<HashMap<String, RhaiMarkdownDocumentCollection>>,
}

impl ComponentContext {
    pub fn get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    pub fn link_to(&self, path: &str) -> Result<String, String> {
        let basename = self.resolve_id(path)?;

        if let Some(reference) = self.markdown_document_by_basename.get(&basename) {
            if !reference.front_matter.render {
                return Err(format!(
                    "Document cannot be linked to, because rendering of it is disabled: {basename}"
                ));
            }

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

    fn resolve_id(&self, path: &str) -> Result<String, String> {
        if path.starts_with("#") {
            if let Some(basename) = self
                .markdown_basename_by_id
                .get(match path.strip_prefix('#') {
                    Some(id) => id,
                    None => return Err("Unable to strip prefix from document id".into()),
                })
            {
                Ok(basename.into())
            } else {
                Err(format!("Document with id does not exist: {path}"))
            }
        } else {
            Ok(path.into())
        }
    }

    fn rhai_collection(
        &mut self,
        collection_name: &str,
    ) -> Result<RhaiMarkdownDocumentCollection, Box<EvalAltResult>> {
        if let Some(collection) = self.rhai_markdown_document_collections.get(collection_name) {
            Ok(collection.clone())
        } else {
            Err(format!("Collection is never used in any document: '{collection_name}'").into())
        }
    }

    fn rhai_front_matter(&mut self) -> RhaiFrontMatter {
        RhaiFrontMatter {
            available_collections: self.available_collections.clone(),
            front_matter: self.front_matter.clone(),
        }
    }

    fn rhai_is_current_page(&mut self, other: String) -> Result<bool, Box<EvalAltResult>> {
        let basename = self.resolve_id(&other)?;

        if self.markdown_document_by_basename.contains_key(&basename) {
            Ok(self.reference.basename() == basename)
        } else {
            Err(format!("Document does not exist: {basename}").into())
        }
    }

    fn rhai_is_watching(&mut self) -> bool {
        self.is_watching
    }

    fn rhai_link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.link_to(path)?)
    }

    fn rhai_reference(&mut self) -> RhaiMarkdownDocumentReference {
        RhaiMarkdownDocumentReference {
            front_matter: self.rhai_front_matter(),
            reference: self.reference.clone(),
        }
    }
}

impl CustomType for ComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ComponentContext")
            .with_get("assets", Self::get_assets)
            .with_get("front_matter", Self::rhai_front_matter)
            .with_get("is_watching", Self::rhai_is_watching)
            .with_get("reference", Self::rhai_reference)
            .with_fn("collection", Self::rhai_collection)
            .with_fn("is_current_page", Self::rhai_is_current_page)
            .with_fn("link_to", Self::rhai_link_to);
    }
}
