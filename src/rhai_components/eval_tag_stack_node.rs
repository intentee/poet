use std::sync::Arc;

use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Map;

use super::attribute_value::AttributeValue;
use super::component_registry::ComponentRegsitry;
use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;

pub fn eval_tag_stack_node<'node, TComponentContext>(
    component_context: TComponentContext,
    component_registry: Arc<ComponentRegsitry>,
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
                    component_registry.clone(),
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

                return Ok(result);
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.is_component()
            {
                let props = {
                    let mut props = Map::new();

                    for attribute in &opening_tag.attributes {
                        props.insert(
                            attribute.name.clone().into(),
                            if let Some(value) = &attribute.value {
                                match value {
                                    AttributeValue::Expression(expression_reference) => {
                                        expression_collection
                                            .eval_expression(eval_context, expression_reference)?
                                    }
                                    AttributeValue::Text(text) => text.into(),
                                }
                            } else {
                                true.into()
                            },
                        );
                    }

                    props
                };

                Ok(eval_context
                    .call_fn::<Dynamic>(
                        component_registry
                            .get_global_fn_name(&opening_tag.name)
                            .or_else(|err| {
                                Err(EvalAltResult::ErrorRuntime(
                                    format!("Component not found: {err}").into(),
                                    rhai::Position::NONE,
                                ))
                            })?,
                        (
                            Dynamic::from(component_context.clone()),
                            Dynamic::from_map(props),
                            Dynamic::from(result.clone()),
                        ),
                    )
                    .or_else(|err| {
                        Err(EvalAltResult::ErrorRuntime(
                            format!("Failed to call component function: {err}").into(),
                            rhai::Position::NONE,
                        ))
                    })?
                    .into_string()
                    .or_else(|err| {
                        Err(EvalAltResult::ErrorRuntime(
                            format!("Failed to convert component output to string: {err}").into(),
                            rhai::Position::NONE,
                        ))
                    })?)
            } else {
                Ok(result)
            }
        }
        TagStackNode::Text(text) => Ok(text.clone()),
    }
}
