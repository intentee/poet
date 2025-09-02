use rhai::CustomType;
use rhai::TypeBuilder;

use crate::rhai_markdown_document_tree_node::RhaiMarkdownDocumentTreeNode;

#[derive(Clone)]
pub struct RhaiMarkdownDocumentHierarchy {
    pub hierarchy: Vec<RhaiMarkdownDocumentTreeNode>,
}

impl CustomType for RhaiMarkdownDocumentHierarchy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("RhaiMarkdownDocumentHierarchy");
    }
}

impl From<Vec<RhaiMarkdownDocumentTreeNode>> for RhaiMarkdownDocumentHierarchy {
    fn from(hierarchy: Vec<RhaiMarkdownDocumentTreeNode>) -> Self {
        Self { hierarchy }
    }
}
