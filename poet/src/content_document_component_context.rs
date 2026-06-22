use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::author::Author;
use crate::author_collection::AuthorCollection;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_reference::ContentDocumentReference;
use crate::table_of_contents::TableOfContents;

#[derive(Clone)]
pub struct ContentDocumentComponentContext {
    pub asset_manager: AssetManager,
    pub authors: Vec<Author>,
    pub available_authors: Arc<AuthorCollection>,
    pub available_collections: Arc<HashSet<String>>,
    pub content_document_collections_ranked: Arc<HashMap<String, ContentDocumentCollectionRanked>>,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: ContentDocumentFrontMatter,
    pub is_watching: bool,
    pub reference: ContentDocumentReference,
    pub table_of_contents: Option<TableOfContents>,
}

impl ContentDocumentComponentContext {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self {
            asset_manager: AssetManager::from_esbuild_metafile(
                Arc::new(esbuild_metafile::EsbuildMetaFile::default()),
                crate::asset_path_renderer::AssetPathRenderer {
                    base_path: "/".to_string(),
                },
            ),
            authors: Vec::new(),
            available_authors: Arc::new(AuthorCollection::default()),
            available_collections: Arc::new(HashSet::new()),
            content_document_collections_ranked: Arc::new(HashMap::new()),
            content_document_linker: ContentDocumentLinker::default(),
            front_matter: ContentDocumentFrontMatter::mock("doc"),
            is_watching: false,
            reference: ContentDocumentReference {
                basename_path: "doc".into(),
                front_matter: ContentDocumentFrontMatter::mock("doc"),
                generated_page_base_path: "/".to_string(),
            },
            table_of_contents: None,
        }
    }

    pub fn with_table_of_contents(self, table_of_contents: TableOfContents) -> Self {
        Self {
            asset_manager: self.asset_manager,
            authors: self.authors,
            available_authors: self.available_authors,
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
        self.authors
            .iter()
            .map(|author| rhai::Dynamic::from(author.clone()))
            .collect()
    }

    fn rhai_available_authors(&mut self) -> rhai::Array {
        self.available_authors
            .values()
            .map(|author| rhai::Dynamic::from(author.clone()))
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
            .with_get("available_authors", Self::rhai_available_authors)
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

#[cfg(test)]
mod tests {
    use esbuild_metafile::EsbuildMetaFile;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::content_document_collection::ContentDocumentCollection;
    use crate::content_document_front_matter::collection_placement::CollectionPlacement;
    use crate::content_document_front_matter::collection_placement_list::CollectionPlacementList;
    use crate::content_document_in_collection::ContentDocumentInCollection;

    fn ranked(name: &str) -> Result<ContentDocumentCollectionRanked, anyhow::Error> {
        ContentDocumentCollection {
            documents: vec![ContentDocumentInCollection {
                collection_placement: CollectionPlacement {
                    after: None,
                    name: name.to_string(),
                    parent: None,
                },
                reference: ContentDocumentReference {
                    basename_path: "entry".into(),
                    front_matter: ContentDocumentFrontMatter::mock("entry"),
                    generated_page_base_path: "/".to_string(),
                },
            }],
            name: name.to_string(),
        }
        .try_into()
    }

    fn ranked_map(
        names: &[&str],
    ) -> Result<HashMap<String, ContentDocumentCollectionRanked>, anyhow::Error> {
        let mut map = HashMap::new();

        for name in names {
            map.insert(name.to_string(), ranked(name)?);
        }

        Ok(map)
    }

    fn front_matter(
        placements: &[&str],
        primary_collection: Option<&str>,
    ) -> ContentDocumentFrontMatter {
        let mut front_matter = ContentDocumentFrontMatter::mock("doc");

        front_matter.collections = CollectionPlacementList {
            placements: placements
                .iter()
                .map(|name| CollectionPlacement {
                    after: None,
                    name: name.to_string(),
                    parent: None,
                })
                .collect(),
        };
        front_matter.primary_collection = primary_collection.map(|name| name.to_string());

        front_matter
    }

    fn context(
        front_matter: ContentDocumentFrontMatter,
        ranked: HashMap<String, ContentDocumentCollectionRanked>,
    ) -> ContentDocumentComponentContext {
        let asset_manager = AssetManager::from_esbuild_metafile(
            Arc::new(EsbuildMetaFile::default()),
            AssetPathRenderer {
                base_path: "/".to_string(),
            },
        );

        ContentDocumentComponentContext {
            asset_manager,
            authors: Vec::new(),
            available_authors: Arc::new(AuthorCollection::default()),
            available_collections: Arc::new(HashSet::new()),
            content_document_collections_ranked: Arc::new(ranked),
            content_document_linker: ContentDocumentLinker::default(),
            front_matter,
            is_watching: false,
            reference: ContentDocumentReference {
                basename_path: "doc".into(),
                front_matter: ContentDocumentFrontMatter::mock("doc"),
                generated_page_base_path: "/".to_string(),
            },
            table_of_contents: None,
        }
    }

    #[test]
    fn collection_returns_ranked_collection_when_used() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&[], None), ranked_map(&["guide"])?);

        assert_eq!(context.rhai_collection("guide")?.name, "guide");

        Ok(())
    }

    #[test]
    fn collection_fails_for_unused_collection() {
        let mut context = context(front_matter(&[], None), HashMap::new());

        assert!(context.rhai_collection("ghost").is_err());
    }

    #[test]
    fn belongs_to_is_true_for_member_collection() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&["guide"], None), ranked_map(&["guide"])?);

        assert!(context.rhai_belongs_to("guide")?);

        Ok(())
    }

    #[test]
    fn belongs_to_is_false_for_non_member_existing_collection() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&[], None), ranked_map(&["guide"])?);

        assert!(!context.rhai_belongs_to("guide")?);

        Ok(())
    }

    #[test]
    fn primary_collection_fails_without_any_placement() {
        let mut context = context(front_matter(&[], None), HashMap::new());

        assert!(context.rhai_primary_collection().is_err());
    }

    #[test]
    fn primary_collection_returns_sole_placement() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&["guide"], None), ranked_map(&["guide"])?);

        assert_eq!(context.rhai_primary_collection()?.name, "guide");

        Ok(())
    }

    #[test]
    fn primary_collection_resolves_declared_primary_among_many() -> Result<(), anyhow::Error> {
        let mut context = context(
            front_matter(&["guide", "reference"], Some("reference")),
            ranked_map(&["guide", "reference"])?,
        );

        assert_eq!(context.rhai_primary_collection()?.name, "reference");

        Ok(())
    }

    #[test]
    fn primary_collection_fails_for_many_without_declared_primary() {
        let mut context = context(front_matter(&["guide", "reference"], None), HashMap::new());

        assert!(context.rhai_primary_collection().is_err());
    }

    #[test]
    fn is_current_page_compares_resolved_basename() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&[], None), HashMap::new());

        assert!(context.rhai_is_current_page("doc".to_string())?);
        assert!(!context.rhai_is_current_page("other".to_string())?);

        Ok(())
    }

    #[test]
    fn table_of_contents_fails_when_absent() {
        let mut context = context(front_matter(&[], None), HashMap::new());

        assert!(context.rhai_table_of_contents().is_err());
    }

    #[test]
    fn table_of_contents_is_available_after_being_set() -> Result<(), anyhow::Error> {
        let mut context = context(front_matter(&[], None), HashMap::new()).with_table_of_contents(
            TableOfContents {
                headings: Vec::new(),
            },
        );

        assert!(context.rhai_table_of_contents().is_ok());

        Ok(())
    }
}
