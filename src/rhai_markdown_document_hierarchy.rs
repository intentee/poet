use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::rhai_markdown_document_reference::RhaiMarkdownDocumentReference;
use crate::rhai_markdown_document_tree_node::RhaiMarkdownDocumentTreeNode;

#[derive(Clone)]
pub struct RhaiMarkdownDocumentHierarchy {
    pub hierarchy: Vec<RhaiMarkdownDocumentTreeNode>,
}

impl RhaiMarkdownDocumentHierarchy {
    fn flatten(&self) -> Vec<RhaiMarkdownDocumentReference> {
        let mut flat: Vec<RhaiMarkdownDocumentReference> = Vec::new();

        for node in &self.hierarchy {
            flat.append(&mut node.flatten());
        }

        flat
    }

    fn rhai_after(&mut self, basename: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut flat_peekable = self
            .flatten()
            .into_iter()
            .filter(|node| node.front_matter.front_matter.render)
            .peekable();

        while let Some(node) = flat_peekable.next() {
            if node.reference.basename() == basename {
                let next: Option<&RhaiMarkdownDocumentReference> = flat_peekable.peek();

                if let Some(next) = next {
                    return Ok(Dynamic::from(next.clone()));
                } else {
                    return Ok(Dynamic::UNIT);
                }
            }
        }

        Err(format!("Next page is not used in the hierarchy: '{basename}'").into())
    }

    fn rhai_before(&mut self, basename: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut previous: Option<RhaiMarkdownDocumentReference> = None;

        for node in self.flatten() {
            if node.front_matter.front_matter.render {
                if node.reference.basename() == basename {
                    if let Some(previous) = previous {
                        return Ok(Dynamic::from(previous));
                    } else {
                        return Ok(Dynamic::UNIT);
                    }
                }

                previous = Some(node.clone());
            }
        }

        Err(format!("Prev page is not used in the hierarchy: '{basename}'").into())
    }
}

impl CustomType for RhaiMarkdownDocumentHierarchy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiMarkdownDocumentHierarchy")
            .with_fn("after", Self::rhai_after)
            .with_fn("before", Self::rhai_before);
    }
}

impl From<Vec<RhaiMarkdownDocumentTreeNode>> for RhaiMarkdownDocumentHierarchy {
    fn from(hierarchy: Vec<RhaiMarkdownDocumentTreeNode>) -> Self {
        Self { hierarchy }
    }
}
