use rhai::EvalAltResult;
use rhai::EvalContext;

use super::attribute_value::AttributeValue;
use super::expression_collection::ExpressionCollection;
use super::tag::Tag;
use super::tag_kind::TagKind;
use crate::escape_html_attribute::escape_html_attribute;

pub fn eval_tag(
    eval_context: &mut EvalContext,
    expression_collection: &ExpressionCollection,
    tag: &Tag,
) -> Result<String, Box<EvalAltResult>> {
    let mut rendered_tag = format!("<{}", tag.tag_name.name);

    for attribute in &tag.attributes {
        rendered_tag.push(' ');
        rendered_tag.push_str(&attribute.name);

        match &attribute.value {
            AttributeValue::Empty => {}
            AttributeValue::Expression(expression_reference) => {
                let attribute_value =
                    expression_collection.eval_expression(eval_context, expression_reference)?;

                rendered_tag.push_str("=\"");
                rendered_tag.push_str(&escape_html_attribute(&attribute_value.to_string()));
                rendered_tag.push('"');
            }
            AttributeValue::Text(text) => {
                rendered_tag.push_str("=\"");
                rendered_tag.push_str(text);
                rendered_tag.push('"');
            }
        }
    }

    match tag.kind {
        TagKind::SelfClosing if !tag.tag_name.is_void_element() => rendered_tag.push_str(" />"),
        TagKind::Closing | TagKind::Opening | TagKind::SelfClosing => rendered_tag.push('>'),
    }

    Ok(rendered_tag)
}
