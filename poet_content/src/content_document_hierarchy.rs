use rhai::Array;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_tree_node::ContentDocumentTreeNode;
use crate::content_error::ContentError;

fn neighbour_of(
    basename: &ContentDocumentBasename,
    rendered_references: impl Iterator<Item = ContentDocumentReference>,
) -> Result<Dynamic, ContentError> {
    let mut rendered_references = rendered_references.peekable();

    while let Some(reference) = rendered_references.next() {
        if reference.basename() == *basename {
            return Ok(rendered_references
                .peek()
                .cloned()
                .map_or(Dynamic::UNIT, Dynamic::from));
        }
    }

    Err(ContentError::HierarchyDocumentNotFound {
        basename: basename.clone(),
    })
}

#[derive(Clone)]
pub struct ContentDocumentHierarchy {
    pub flat: Vec<ContentDocumentReference>,
    pub roots: Vec<ContentDocumentTreeNode>,
}

impl ContentDocumentHierarchy {
    fn rendered_references(&self) -> impl DoubleEndedIterator<Item = ContentDocumentReference> {
        self.flat
            .iter()
            .filter(|reference| reference.front_matter.render)
            .cloned()
    }

    fn rhai_after(&mut self, basename: String) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(neighbour_of(
            &ContentDocumentBasename(basename),
            self.rendered_references(),
        )?)
    }

    fn rhai_before(&mut self, basename: String) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(neighbour_of(
            &ContentDocumentBasename(basename),
            self.rendered_references().rev(),
        )?)
    }

    fn rhai_flat(&mut self) -> Array {
        self.flat.iter().cloned().map(Dynamic::from).collect()
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
        Self {
            flat: roots
                .iter()
                .flat_map(ContentDocumentTreeNode::flatten)
                .collect(),
            roots,
        }
    }
}
