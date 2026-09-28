use poet_assets::asset_manager::AssetManager;
use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_context::ContentDocumentContext;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_error::ContentError;
use crate::content_site_context::ContentSiteContext;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents_state::TableOfContentsState;

#[derive(Clone)]
pub struct ContentDocumentComponentContext {
    pub asset_manager: AssetManager,
    pub document: ContentDocumentContext,
    pub site: ContentSiteContext,
    pub table_of_contents: TableOfContentsState,
}

impl ContentDocumentComponentContext {
    #[must_use]
    pub fn with_table_of_contents(self, table_of_contents: TableOfContents) -> Self {
        Self {
            table_of_contents: TableOfContentsState::Collected(table_of_contents),
            ..self
        }
    }

    fn collection(
        &self,
        collection_name: &str,
    ) -> Result<ContentDocumentCollectionRanked, ContentError> {
        self.site
            .content_document_collections_ranked
            .get(collection_name)
            .cloned()
            .ok_or_else(|| ContentError::CollectionNotUsed {
                collection_name: collection_name.to_owned(),
            })
    }

    fn primary_collection(&self) -> Result<ContentDocumentCollectionRanked, ContentError> {
        let front_matter = &self.document.reference.front_matter;
        let primary_collection_name = match front_matter.collections.as_slice() {
            [] => Err(ContentError::DocumentHasNoCollection {
                basename: self.document.reference.basename(),
            }),
            [collection_placement] => Ok(&collection_placement.name),
            _ => front_matter.primary_collection.as_ref().ok_or_else(|| {
                ContentError::PrimaryCollectionNotDeclared {
                    basename: self.document.reference.basename(),
                }
            }),
        }?;

        self.collection(primary_collection_name)
    }

    fn rhai_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    fn rhai_authors(&mut self) -> Array {
        self.document
            .authors
            .iter()
            .cloned()
            .map(Dynamic::from)
            .collect()
    }

    fn rhai_available_authors(&mut self) -> Array {
        self.site
            .available_authors
            .values()
            .cloned()
            .map(Dynamic::from)
            .collect()
    }

    fn rhai_belongs_to(&mut self, collection_name: &str) -> Result<bool, Box<EvalAltResult>> {
        self.collection(collection_name)?;

        Ok(self
            .document
            .reference
            .front_matter
            .collections
            .iter()
            .any(|collection_placement| collection_placement.name == collection_name))
    }

    fn rhai_collection(
        &mut self,
        collection_name: &str,
    ) -> Result<ContentDocumentCollectionRanked, Box<EvalAltResult>> {
        Ok(self.collection(collection_name)?)
    }

    fn rhai_front_matter(&mut self) -> ContentDocumentFrontMatter {
        self.document.reference.front_matter.as_ref().clone()
    }

    fn rhai_is_current_page(&mut self, link: &str) -> Result<bool, Box<EvalAltResult>> {
        Ok(self.site.content_document_linker.resolve_basename(link)?
            == self.document.reference.basename())
    }

    const fn rhai_is_watching(&mut self) -> bool {
        self.site.is_watching
    }

    fn rhai_link_to(&mut self, link: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.site.content_document_linker.link_to(link)?)
    }

    fn rhai_primary_collection(
        &mut self,
    ) -> Result<ContentDocumentCollectionRanked, Box<EvalAltResult>> {
        Ok(self.primary_collection()?)
    }

    fn rhai_reference(&mut self) -> ContentDocumentReference {
        self.document.reference.clone()
    }

    fn rhai_table_of_contents(&mut self) -> Result<TableOfContents, Box<EvalAltResult>> {
        match &self.table_of_contents {
            TableOfContentsState::BeingCollected => {
                Err(ContentError::TableOfContentsUnavailable.into())
            }
            TableOfContentsState::Collected(table_of_contents) => Ok(table_of_contents.clone()),
        }
    }
}

impl CustomType for ContentDocumentComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentComponentContext")
            .with_get("assets", Self::rhai_assets)
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
