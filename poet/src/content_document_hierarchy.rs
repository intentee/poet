use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_tree_node::ContentDocumentTreeNode;

#[derive(Clone)]
pub struct ContentDocumentHierarchy {
    pub flat: Vec<ContentDocumentReference>,
    pub roots: Vec<ContentDocumentTreeNode>,
}

impl ContentDocumentHierarchy {
    fn rhai_after(&mut self, basename_string: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let basename: ContentDocumentBasename = basename_string.into();
        let mut flat_peekable = self
            .flat
            .clone()
            .into_iter()
            .filter(|node| node.front_matter.render)
            .peekable();

        while let Some(node) = flat_peekable.next() {
            if node.basename() == basename {
                let next: Option<&ContentDocumentReference> = flat_peekable.peek();

                if let Some(next) = next {
                    return Ok(Dynamic::from(next.clone()));
                } else {
                    return Ok(Dynamic::UNIT);
                }
            }
        }

        Err(format!("Next page is not used in the hierarchy: '{basename}'").into())
    }

    fn rhai_before(&mut self, basename_string: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let basename: ContentDocumentBasename = basename_string.into();
        let mut previous: Option<ContentDocumentReference> = None;

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

    fn rhai_flat(&mut self) -> Array {
        self.flat
            .iter()
            .map(|content_document_reference| Dynamic::from(content_document_reference.clone()))
            .collect::<_>()
    }
}

impl CustomType for ContentDocumentHierarchy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentHierarchy")
            .with_get("flat", Self::rhai_flat)
            .with_fn("after", Self::rhai_after)
            .with_fn("before", Self::rhai_before);
    }
}

impl From<Vec<ContentDocumentTreeNode>> for ContentDocumentHierarchy {
    fn from(roots: Vec<ContentDocumentTreeNode>) -> Self {
        let mut flat: Vec<ContentDocumentReference> = Vec::new();

        for node in &roots {
            flat.append(&mut node.flatten());
        }

        Self { flat, roots }
    }
}
