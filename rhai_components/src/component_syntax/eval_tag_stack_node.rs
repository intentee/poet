use std::sync::Arc;

use rhai::Array;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::ImmutableString;
use rhai::Map;

use super::attribute_value::AttributeValue;
use super::component_registry::ComponentRegistry;
use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;
use crate::SmartStringLazy;
use crate::rhai_call_template_function::rhai_call_template_function;

pub fn eval_tag_stack_node(
    component_registry: Arc<ComponentRegistry>,
    eval_context: &mut EvalContext,
    current_node: &TagStackNode,
    expression_collection: &mut ExpressionCollection,
) -> Result<SmartStringLazy, Box<EvalAltResult>> {
    match current_node {
        TagStackNode::BodyExpression(expression_reference) => {
            let body_expression_result =
                expression_collection.eval_expression(eval_context, expression_reference)?;

            if body_expression_result.is_array() {
                let body_expression_array = body_expression_result.cast::<Array>();
                let mut combined_ret = SmartStringLazy::new_const();

                for item in body_expression_array {
                    combined_ret.push_str(&item.to_string());
                }

                Ok(combined_ret)
            } else if body_expression_result.is_string() {
                let body_expression_string = body_expression_result.cast::<ImmutableString>();

                Ok(body_expression_string.into())
            } else {
                Ok(body_expression_result.to_string().into())
            }
        }
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let mut result = SmartStringLazy::new_const();

            if let Some(opening_tag) = &opening_tag
                && !opening_tag.tag_name.is_component()
            {
                result.push_str(&eval_tag(eval_context, expression_collection, opening_tag)?);
            }

            for child in children {
                result.push_str(&eval_tag_stack_node(
                    component_registry.clone(),
                    eval_context,
                    child,
                    expression_collection,
                )?);
            }

            if let Some(opening_tag) = &opening_tag
                && *is_closed
                && !opening_tag.tag_name.is_component()
            {
                result.push_str(&format!("</{}>", opening_tag.tag_name.name));

                return Ok(result);
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.tag_name.is_component()
            {
                let props = {
                    let mut props = Map::new();

                    for attribute in &opening_tag.attributes {
                        props.insert(
                            attribute.name.clone(),
                            if let Some(value) = &attribute.value {
                                match value {
                                    AttributeValue::Expression(expression_reference) => {
                                        expression_collection
                                            .eval_expression(eval_context, expression_reference)?
                                    }
                                    AttributeValue::Text(text) => text.into(),
                                }
                            } else {
                                Dynamic::TRUE
                            },
                        );
                    }

                    props
                };

                let Some(context) = eval_context.scope().get("context").cloned() else {
                    return Err(EvalAltResult::ErrorRuntime(
                        "'context' variable not found in scope".into(),
                        rhai::Position::NONE,
                    )
                    .into());
                };

                Ok(rhai_call_template_function(
                    eval_context.engine(),
                    &opening_tag.tag_name.name,
                    (
                        context,
                        Dynamic::from_map(props),
                        Dynamic::from(result.to_string()),
                    ),
                )
                .map_err(|err| {
                    EvalAltResult::ErrorRuntime(
                        format!("Failed to call component function: {err}").into(),
                        rhai::Position::NONE,
                    )
                })?)
            } else {
                Ok(result)
            }
        }
        TagStackNode::Text(text) => Ok(text.clone()),
    }
}
