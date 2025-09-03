use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::FnPtr;
use rhai::NativeCallContext;

use crate::markdown_document_hierarchy::MarkdownDocumentHierarchy;
use crate::markdown_document_tree_node::MarkdownDocumentTreeNode;

fn render_node(
    context: &NativeCallContext,
    node: &MarkdownDocumentTreeNode,
    callback: &FnPtr,
    nesting_level: i64,
) -> Result<Dynamic, Box<EvalAltResult>> {
    callback.call_within_context(
        context,
        (
            Dynamic::from(node.clone()),
            Dynamic::from_int(nesting_level),
            {
                let mut next_level = String::new();

                for child in &node.children {
                    next_level.push_str(
                        &render_node(context, child, callback, nesting_level + 1)?.to_string(),
                    );
                }

                Dynamic::from(next_level)
            },
        ),
    )
}

pub fn render_hierarchy(
    context: NativeCallContext,
    hierarchy: MarkdownDocumentHierarchy,
    callback: FnPtr,
) -> Result<String, Box<EvalAltResult>> {
    let results = hierarchy
        .hierarchy
        .iter()
        .map(|node| render_node(&context, node, &callback, 0))
        .collect::<Result<Vec<Dynamic>, Box<EvalAltResult>>>()?;

    let mut rendered = String::new();

    for result in results {
        rendered.push_str(&result.to_string());
    }

    Ok(rendered)
}
