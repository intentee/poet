use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;

use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;

pub fn eval_tag_stack_node<'node, TComponentContext>(
    component_context: TComponentContext,
    eval_context: &mut EvalContext,
    current_node: &'node TagStackNode,
    expression_collection: &mut ExpressionCollection,
) -> Result<String, Box<EvalAltResult>>
where
    TComponentContext: Clone + Send + Sync + 'static,
{
    match current_node {
        TagStackNode::BodyExpression(expression_reference) => Ok(expression_collection
            .eval_expression(eval_context, expression_reference)?
            .into_string()?),
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let mut result = String::new();

            if let Some(opening_tag) = &opening_tag
                && !opening_tag.is_component()
            {
                result.push_str(&eval_tag(eval_context, expression_collection, opening_tag)?);
            }

            for child in children {
                result.push_str(&eval_tag_stack_node(
                    component_context.clone(),
                    eval_context,
                    child,
                    expression_collection,
                )?);
            }

            if let Some(opening_tag) = &opening_tag
                && *is_closed
                && !opening_tag.is_component()
            {
                result.push_str(&format!("</{}>", opening_tag.name));
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.is_component()
            {
                Ok(eval_context
                    .call_fn::<Dynamic>(
                        "LayoutHomepage_123",
                        (
                            Dynamic::from(component_context.clone()),
                            Dynamic::from(""),
                            Dynamic::from(result.clone()),
                        ),
                    )?
                    .into_string()?)
            } else {
                Ok(result)
            }
        }
        TagStackNode::Text(text) => Ok(text.clone()),
    }
}
