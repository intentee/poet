use std::collections::LinkedList;

use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone)]
pub struct MarkdownDocumentTreeNode {
    pub children: LinkedList<MarkdownDocumentTreeNode>,
    pub reference: MarkdownDocumentReference,
}

impl MarkdownDocumentTreeNode {
    pub fn flatten(&self) -> Vec<MarkdownDocumentReference> {
        let mut flat: Vec<MarkdownDocumentReference> = Vec::new();

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

    fn rhai_reference(&mut self) -> MarkdownDocumentReference {
        self.reference.clone()
    }
}

impl CustomType for MarkdownDocumentTreeNode {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("MarkdownDocumentTreeNode")
            .with_get("children", Self::rhai_children)
            .with_get("reference", Self::rhai_reference);
    }
}
