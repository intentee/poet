use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone)]
pub struct ContentDocumentTreeNode {
    pub children: Vec<Self>,
    pub collection_name: String,
    pub reference: ContentDocumentReference,
}

impl ContentDocumentTreeNode {
    #[must_use]
    pub fn flatten(&self) -> Vec<ContentDocumentReference> {
        let mut flat_references = vec![self.reference.clone()];

        for child in &self.children {
            flat_references.append(&mut child.flatten());
        }

        flat_references
    }

    fn rhai_children(&mut self) -> Array {
        self.children.iter().cloned().map(Dynamic::from).collect()
    }

    fn rhai_collection_name(&mut self) -> String {
        self.collection_name.clone()
    }

    fn rhai_reference(&mut self) -> ContentDocumentReference {
        self.reference.clone()
    }
}

impl CustomType for ContentDocumentTreeNode {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentTreeNode")
            .with_get("children", Self::rhai_children)
            .with_get("collection_name", Self::rhai_collection_name)
            .with_get("reference", Self::rhai_reference);
    }
}
