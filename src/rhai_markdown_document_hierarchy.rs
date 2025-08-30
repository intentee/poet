use rhai::CustomType;
use rhai::TypeBuilder;

use crate::markdown_document_tree_node::MarkdownDocumentTreeNode;

#[derive(Clone, Debug)]
pub struct RhaiMarkdownDocumentHierarchy {
    pub hierarchy: Vec<MarkdownDocumentTreeNode>,
}

impl CustomType for RhaiMarkdownDocumentHierarchy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("RhaiMarkdownDocumentHierarchy");
    }
}

impl From<Vec<MarkdownDocumentTreeNode>> for RhaiMarkdownDocumentHierarchy {
    fn from(hierarchy: Vec<MarkdownDocumentTreeNode>) -> Self {
        Self { hierarchy }
    }
}
