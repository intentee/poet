use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::markdown_document_tree_node::MarkdownDocumentTreeNode;

#[derive(Clone)]
pub struct MarkdownDocumentHierarchy {
    pub flat: Vec<MarkdownDocumentReference>,
    pub roots: Vec<MarkdownDocumentTreeNode>,
}

impl MarkdownDocumentHierarchy {
    fn rhai_after(&mut self, basename: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut flat_peekable = self
            .flat
            .clone()
            .into_iter()
            .filter(|node| node.front_matter.render)
            .peekable();

        while let Some(node) = flat_peekable.next() {
            if node.basename() == basename {
                let next: Option<&MarkdownDocumentReference> = flat_peekable.peek();

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
        let mut previous: Option<MarkdownDocumentReference> = None;

        for node in &self.flat {
            if node.front_matter.render {
                if node.basename() == basename {
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

impl CustomType for MarkdownDocumentHierarchy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("MarkdownDocumentHierarchy")
            .with_fn("after", Self::rhai_after)
            .with_fn("before", Self::rhai_before);
    }
}

impl From<Vec<MarkdownDocumentTreeNode>> for MarkdownDocumentHierarchy {
    fn from(roots: Vec<MarkdownDocumentTreeNode>) -> Self {
        let mut flat: Vec<MarkdownDocumentReference> = Vec::new();

        for node in &roots {
            flat.append(&mut node.flatten());
        }

        Self { flat, roots }
    }
}
