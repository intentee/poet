use rhai::CustomType;
use rhai::TypeBuilder;

use crate::content_document_collection::ContentDocumentCollection;
use crate::content_document_hierarchy::ContentDocumentHierarchy;
use crate::content_error::ContentError;

#[derive(Clone)]
pub struct ContentDocumentCollectionRanked {
    pub hierarchy: ContentDocumentHierarchy,
    pub name: String,
}

impl ContentDocumentCollectionRanked {
    fn rhai_hierarchy(&mut self) -> ContentDocumentHierarchy {
        self.hierarchy.clone()
    }

    fn rhai_name(&mut self) -> String {
        self.name.clone()
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

impl TryFrom<&ContentDocumentCollection> for ContentDocumentCollectionRanked {
    type Error = ContentError;

    fn try_from(collection: &ContentDocumentCollection) -> Result<Self, Self::Error> {
        Ok(Self {
            hierarchy: ContentDocumentHierarchy::from(collection.build_hierarchy()?),
            name: collection.name.clone(),
        })
    }
}
