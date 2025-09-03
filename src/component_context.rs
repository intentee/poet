use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::front_matter::FrontMatter;
use crate::markdown_document_collection::MarkdownDocumentCollection;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::table_of_contents::TableOfContents;

#[derive(Clone)]
pub struct ComponentContext {
    pub asset_manager: AssetManager,
    pub available_collections: Arc<HashSet<String>>,
    pub front_matter: FrontMatter,
    pub is_watching: bool,
    pub markdown_basename_by_id: Arc<HashMap<String, String>>,
    pub markdown_document_by_basename: Arc<HashMap<String, MarkdownDocumentReference>>,
    pub markdown_document_collections: Arc<HashMap<String, MarkdownDocumentCollection>>,
    pub reference: MarkdownDocumentReference,
    pub table_of_contents: Option<TableOfContents>,
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

    pub fn with_table_of_contents(self, table_of_contents: TableOfContents) -> Self {
        Self {
            asset_manager: self.asset_manager,
            available_collections: self.available_collections,
            front_matter: self.front_matter,
            is_watching: self.is_watching,
            markdown_basename_by_id: self.markdown_basename_by_id,
            markdown_document_by_basename: self.markdown_document_by_basename,
            markdown_document_collections: self.markdown_document_collections,
            reference: self.reference,
            table_of_contents: Some(table_of_contents),
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

    fn rhai_belongs_to(&mut self, collection_name: &str) -> Result<bool, Box<EvalAltResult>> {
        // This validates that the colllection actually exists anywhere
        let _ = self.rhai_collection(collection_name)?;

        for placement in &self.front_matter.collections.placements {
            if placement.name == collection_name {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn rhai_collection(
        &mut self,
        collection_name: &str,
    ) -> Result<MarkdownDocumentCollection, Box<EvalAltResult>> {
        if let Some(collection) = self.markdown_document_collections.get(collection_name) {
            Ok(collection.clone())
        } else {
            Err(format!("Collection is never used in any document: '{collection_name}'").into())
        }
    }

    fn rhai_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
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

    fn rhai_primary_collection(
        &mut self,
    ) -> Result<MarkdownDocumentCollection, Box<EvalAltResult>> {
        if self.front_matter.collections.placements.len() == 1 {
            let placements = self.front_matter.collections.placements.clone();

            if let Some(placement) = placements.first() {
                return self.rhai_collection(&placement.name);
            }
        }

        Err("Unable to determine the primary collection".into())
    }

    fn rhai_reference(&mut self) -> MarkdownDocumentReference {
        self.reference.clone()
    }

    fn rhai_table_of_contents(&mut self) -> Result<TableOfContents, Box<EvalAltResult>> {
        if let Some(table_of_contents) = &self.table_of_contents {
            Ok(table_of_contents.clone())
        } else {
            Err("Table of contents is not available. Do not use table of contents variable in document headers.".into())
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
            .with_get("primary_collection", Self::rhai_primary_collection)
            .with_get("reference", Self::rhai_reference)
            .with_get("table_of_contents", Self::rhai_table_of_contents)
            .with_fn("belongs_to", Self::rhai_belongs_to)
            .with_fn("collection", Self::rhai_collection)
            .with_fn("is_current_page", Self::rhai_is_current_page)
            .with_fn("link_to", Self::rhai_link_to);
    }
}
