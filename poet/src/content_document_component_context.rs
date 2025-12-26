use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::author::Author;
use crate::author_basename::AuthorBasename;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_reference::ContentDocumentReference;
use crate::table_of_contents::TableOfContents;

#[derive(Clone)]
pub struct ContentDocumentComponentContext {
    pub asset_manager: AssetManager,
    pub authors: Arc<BTreeMap<AuthorBasename, Author>>,
    pub available_collections: Arc<HashSet<String>>,
    pub content_document_collections_ranked: Arc<HashMap<String, ContentDocumentCollectionRanked>>,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: ContentDocumentFrontMatter,
    pub is_watching: bool,
    pub reference: ContentDocumentReference,
    pub table_of_contents: Option<TableOfContents>,
}

impl ContentDocumentComponentContext {
    pub fn with_table_of_contents(self, table_of_contents: TableOfContents) -> Self {
        Self {
            asset_manager: self.asset_manager,
            authors: self.authors,
            available_collections: self.available_collections,
            content_document_collections_ranked: self.content_document_collections_ranked,
            content_document_linker: self.content_document_linker,
            front_matter: self.front_matter,
            is_watching: self.is_watching,
            reference: self.reference,
            table_of_contents: Some(table_of_contents),
        }
    }

    fn rhai_authors(&mut self) -> rhai::Array {
        self.front_matter
            .authors
            .iter()
            .filter_map(|basename| self.authors.get(basename))
            .map(|author| rhai::Dynamic::from(author.front_matter.clone()))
            .collect()
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
    ) -> Result<ContentDocumentCollectionRanked, Box<EvalAltResult>> {
        if let Some(collection) = self
            .content_document_collections_ranked
            .get(collection_name)
        {
            Ok(collection.clone())
        } else {
            Err(format!("Collection is never used in any document: '{collection_name}'").into())
        }
    }

    fn rhai_front_matter(&mut self) -> ContentDocumentFrontMatter {
        self.front_matter.clone()
    }

    pub fn rhai_get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    fn rhai_is_current_page(&mut self, other: String) -> Result<bool, Box<EvalAltResult>> {
        let basename = self.content_document_linker.resolve_id(&other)?;

        Ok(self.reference.basename() == basename)
    }

    fn rhai_is_watching(&mut self) -> bool {
        self.is_watching
    }

    fn rhai_link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.content_document_linker.link_to(path)?)
    }

    fn rhai_primary_collection(
        &mut self,
    ) -> Result<ContentDocumentCollectionRanked, Box<EvalAltResult>> {
        match self.front_matter.collections.placements.len() {
            0 => return Err("Document does not belong to any collection".into()),
            1 => {
                let placements = self.front_matter.collections.placements.clone();

                if let Some(placement) = placements.first() {
                    return self.rhai_collection(&placement.name);
                }
            }
            _ => {
                if let Some(primary_collection) = &self.front_matter.primary_collection {
                    let placements = self.front_matter.collections.placements.clone();

                    for placement in placements {
                        if placement.name == *primary_collection {
                            return self.rhai_collection(&placement.name);
                        }
                    }
                } else {
                    return Err("Document has multiple collections, but it doesn't specify the primary collection (which normally isn't a problem, but you tried to use the '.primary_collection' field)".into());
                }
            }
        };

        Err("Unable to determine the primary collection".into())
    }

    fn rhai_reference(&mut self) -> ContentDocumentReference {
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

impl CustomType for ContentDocumentComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentComponentContext")
            .with_get("assets", Self::rhai_get_assets)
            .with_get("authors", Self::rhai_authors)
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
