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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::content_document_front_matter::ContentDocumentFrontMatter;

    fn reference(basename: &str, render: bool) -> ContentDocumentReference {
        let mut front_matter = ContentDocumentFrontMatter::mock(basename);

        front_matter.render = render;

        ContentDocumentReference {
            basename_path: basename.into(),
            front_matter,
            generated_page_base_path: "/".to_string(),
        }
    }

    fn tree_node(
        basename: &str,
        render: bool,
        children: Vec<ContentDocumentTreeNode>,
    ) -> ContentDocumentTreeNode {
        ContentDocumentTreeNode {
            children: children.into_iter().collect(),
            collection_name: "docs".to_string(),
            reference: reference(basename, render),
        }
    }

    fn hierarchy() -> ContentDocumentHierarchy {
        ContentDocumentHierarchy::from(vec![
            tree_node("a", true, vec![]),
            tree_node("hidden", false, vec![]),
            tree_node("b", true, vec![]),
        ])
    }

    fn basename_of(value: Dynamic) -> Option<String> {
        value
            .try_cast::<ContentDocumentReference>()
            .map(|reference| reference.basename().to_string())
    }

    #[test]
    fn flattens_tree_depth_first() {
        let hierarchy = ContentDocumentHierarchy::from(vec![
            tree_node("a", true, vec![tree_node("a-child", true, vec![])]),
            tree_node("b", true, vec![]),
        ]);

        let basenames: Vec<String> = hierarchy
            .flat
            .iter()
            .map(|reference| reference.basename().to_string())
            .collect();

        assert_eq!(
            basenames,
            vec!["a".to_string(), "a-child".to_string(), "b".to_string()]
        );
    }

    #[test]
    fn after_returns_next_renderable_document_skipping_hidden() -> Result<()> {
        assert_eq!(
            basename_of(hierarchy().rhai_after("a".to_string())?),
            Some("b".to_string())
        );

        Ok(())
    }

    #[test]
    fn after_returns_unit_for_last_document() -> Result<()> {
        assert!(hierarchy().rhai_after("b".to_string())?.is_unit());

        Ok(())
    }

    #[test]
    fn after_fails_for_document_not_in_hierarchy() {
        assert!(hierarchy().rhai_after("missing".to_string()).is_err());
    }

    #[test]
    fn before_returns_previous_renderable_document_skipping_hidden() -> Result<()> {
        assert_eq!(
            basename_of(hierarchy().rhai_before("b".to_string())?),
            Some("a".to_string())
        );

        Ok(())
    }

    #[test]
    fn before_returns_unit_for_first_document() -> Result<()> {
        assert!(hierarchy().rhai_before("a".to_string())?.is_unit());

        Ok(())
    }

    #[test]
    fn before_fails_for_document_not_in_hierarchy() {
        assert!(hierarchy().rhai_before("missing".to_string()).is_err());
    }
}
