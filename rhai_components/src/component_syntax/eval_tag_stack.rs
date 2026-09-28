use rhai::Array;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Map;

use super::attribute_value::AttributeValue;
use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::expression_reference::ExpressionReference;
use super::tag_kind::TagKind;
use super::tag_stack::TagStack;
use super::tag_stack_element::TagStackElement;
use super::tag_stack_node::TagStackNode;
use crate::component_context_variable_name::COMPONENT_CONTEXT_VARIABLE_NAME;
use crate::component_nesting_depth::ComponentNestingDepth;
use crate::rhai_call_template_function::rhai_call_template_function;
use crate::rhai_components_error::RhaiComponentsError;

fn eval_body_expression(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    expression_reference: &ExpressionReference,
) -> Result<String, Box<EvalAltResult>> {
    let body_expression_result =
        expression_collection.eval_expression(eval_context, expression_reference)?;

    Ok(body_expression_result.read_lock::<Array>().map_or_else(
        || body_expression_result.to_string(),
        |body_expression_array| {
            body_expression_array
                .iter()
                .map(ToString::to_string)
                .collect()
        },
    ))
}

fn eval_component_props(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    element: &TagStackElement,
) -> Result<Map, Box<EvalAltResult>> {
    let mut component_props = Map::new();

    for attribute in &element.opening_tag.attributes {
        let prop_value = match &attribute.value {
            AttributeValue::Empty => true.into(),
            AttributeValue::Expression(expression_reference) => {
                expression_collection.eval_expression(eval_context, expression_reference)?
            }
            AttributeValue::Text(text) => text.into(),
        };

        component_props.insert(attribute.name.clone().into(), prop_value);
    }

    Ok(component_props)
}

fn eval_component(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    element: &TagStackElement,
) -> Result<String, Box<EvalAltResult>> {
    let component_name = &element.opening_tag.tag_name.name;
    let rendered_children = eval_nodes(eval_context, expression_collection, &element.children)?;
    let component_props = eval_component_props(eval_context, expression_collection, element)?;
    let component_context = eval_context
        .scope()
        .get(COMPONENT_CONTEXT_VARIABLE_NAME)
        .cloned()
        .ok_or_else(|| RhaiComponentsError::ComponentContextNotInScope {
            variable_name: COMPONENT_CONTEXT_VARIABLE_NAME.to_owned(),
        })?;
    let component_nesting_depth = eval_context
        .tag()
        .read_lock::<ComponentNestingDepth>()
        .map(|component_nesting_depth| *component_nesting_depth)
        .ok_or(RhaiComponentsError::MissingComponentNestingDepth)?
        .nested(component_name.clone())?;

    Ok(rhai_call_template_function(
        eval_context.engine(),
        component_name,
        component_nesting_depth,
        (
            component_context,
            Dynamic::from_map(component_props),
            Dynamic::from(rendered_children),
        ),
    )?)
}

fn eval_element(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    element: &TagStackElement,
) -> Result<String, Box<EvalAltResult>> {
    if element.opening_tag.tag_name.is_component() {
        return eval_component(eval_context, expression_collection, element);
    }

    let mut rendered_element = eval_tag(eval_context, expression_collection, &element.opening_tag)?;

    rendered_element.push_str(&eval_nodes(
        eval_context,
        expression_collection,
        &element.children,
    )?);

    if element.opening_tag.kind == TagKind::Opening
        && !element.opening_tag.tag_name.is_void_element()
    {
        rendered_element.push_str(&element.opening_tag.tag_name.closing_tag());
    }

    Ok(rendered_element)
}

fn eval_nodes(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    nodes: &[TagStackNode],
) -> Result<String, Box<EvalAltResult>> {
    let mut rendered_nodes = String::new();

    for node in nodes {
        rendered_nodes.push_str(&match node {
            TagStackNode::BodyExpression(expression_reference) => {
                eval_body_expression(eval_context, expression_collection, expression_reference)?
            }
            TagStackNode::Element(element) => {
                eval_element(eval_context, expression_collection, element)?
            }
            TagStackNode::Text(text) => text.clone(),
        });
    }

    Ok(rendered_nodes)
}

pub fn eval_tag_stack(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    tag_stack: &TagStack,
) -> Result<String, Box<EvalAltResult>> {
    eval_nodes(eval_context, expression_collection, &tag_stack.children)
}
