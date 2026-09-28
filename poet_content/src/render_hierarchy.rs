use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::FnPtr;
use rhai::INT;
use rhai::NativeCallContext;

use crate::content_document_hierarchy::ContentDocumentHierarchy;
use crate::content_document_tree_node::ContentDocumentTreeNode;

fn render_nodes(
    native_call_context: &NativeCallContext,
    nodes: &[ContentDocumentTreeNode],
    callback: &FnPtr,
    nesting_level: INT,
) -> Result<String, Box<EvalAltResult>> {
    nodes
        .iter()
        .map(|node| {
            callback
                .call_within_context::<Dynamic>(
                    native_call_context,
                    (
                        Dynamic::from(node.clone()),
                        Dynamic::from_int(nesting_level),
                        Dynamic::from(render_nodes(
                            native_call_context,
                            &node.children,
                            callback,
                            nesting_level + 1,
                        )?),
                    ),
                )
                .map(|rendered_node| rendered_node.to_string())
        })
        .collect()
}

pub fn render_hierarchy(
    native_call_context: NativeCallContext,
    hierarchy: ContentDocumentHierarchy,
    callback: FnPtr,
) -> Result<String, Box<EvalAltResult>> {
    render_nodes(&native_call_context, &hierarchy.roots, &callback, 0)
}
