use std::collections::LinkedList;

use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::TypeBuilder;

use crate::rhai_markdown_document_reference::RhaiMarkdownDocumentReference;

#[derive(Clone)]
pub struct RhaiMarkdownDocumentTreeNode {
    pub children: LinkedList<RhaiMarkdownDocumentTreeNode>,
    pub reference: RhaiMarkdownDocumentReference,
}

impl RhaiMarkdownDocumentTreeNode {
    fn rhai_children(&mut self) -> Array {
        self.children
            .iter()
            .map(|node| Dynamic::from(node.clone()))
            .collect::<Vec<_>>()
    }

    fn rhai_reference(&mut self) -> RhaiMarkdownDocumentReference {
        self.reference.clone()
    }
}

impl CustomType for RhaiMarkdownDocumentTreeNode {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiMarkdownDocumentTreeNode")
            .with_get("children", Self::rhai_children)
            .with_get("reference", Self::rhai_reference);
    }
}
