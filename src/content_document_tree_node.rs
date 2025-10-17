use std::collections::LinkedList;

use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::content_document_reference::ContentDocumentReference;

#[derive(Clone)]
pub struct ContentDocumentTreeNode {
    pub children: LinkedList<ContentDocumentTreeNode>,
    pub collection_name: String,
    pub reference: ContentDocumentReference,
}

impl ContentDocumentTreeNode {
    pub fn flatten(&self) -> Vec<ContentDocumentReference> {
        let mut flat: Vec<ContentDocumentReference> = Vec::new();

        flat.push(self.reference.clone());

        for node in &self.children {
            flat.append(&mut node.flatten());
        }

        flat
    }

    fn rhai_children(&mut self) -> Array {
        self.children
            .iter()
            .map(|node| Dynamic::from(node.clone()))
            .collect::<Vec<_>>()
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
