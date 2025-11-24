use anyhow::Result;
use rhai::CustomType;
use rhai::TypeBuilder;

use crate::content_document_collection::ContentDocumentCollection;
use crate::content_document_hierarchy::ContentDocumentHierarchy;

#[derive(Clone)]
pub struct ContentDocumentCollectionRanked {
    pub name: String,
    pub hierarchy: ContentDocumentHierarchy,
}

impl ContentDocumentCollectionRanked {
    fn rhai_name(&mut self) -> String {
        self.name.clone()
    }

    fn rhai_hierarchy(&mut self) -> ContentDocumentHierarchy {
        self.hierarchy.clone()
    }
}

impl CustomType for ContentDocumentCollectionRanked {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentCollectionRanked")
            .with_get("hierarchy", Self::rhai_hierarchy)
            .with_get("name", Self::rhai_name);
    }
}

impl TryFrom<ContentDocumentCollection> for ContentDocumentCollectionRanked {
    type Error = anyhow::Error;

    fn try_from(collection: ContentDocumentCollection) -> Result<Self> {
        Ok(Self {
            hierarchy: ContentDocumentHierarchy::from(collection.build_hierarchy()?),
            name: collection.name,
        })
    }
}
