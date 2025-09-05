use anyhow::Result;
use rhai::CustomType;
use rhai::TypeBuilder;

use crate::markdown_document_collection::MarkdownDocumentCollection;
use crate::markdown_document_hierarchy::MarkdownDocumentHierarchy;

#[derive(Clone)]
pub struct MarkdownDocumentCollectionRanked {
    pub name: String,
    pub hierarchy: MarkdownDocumentHierarchy,
}

impl MarkdownDocumentCollectionRanked {
    fn rhai_name(&mut self) -> String {
        self.name.clone()
    }

    fn rhai_hierarchy(&mut self) -> MarkdownDocumentHierarchy {
        self.hierarchy.clone()
    }
}

impl CustomType for MarkdownDocumentCollectionRanked {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("MarkdownDocumentCollectionRanked")
            .with_get("hierarchy", Self::rhai_hierarchy)
            .with_get("name", Self::rhai_name);
    }
}

impl TryFrom<MarkdownDocumentCollection> for MarkdownDocumentCollectionRanked {
    type Error = anyhow::Error;

    fn try_from(collection: MarkdownDocumentCollection) -> Result<Self> {
        Ok(Self {
            hierarchy: MarkdownDocumentHierarchy::from(collection.build_hierarchy()?),
            name: collection.name,
        })
    }
}
