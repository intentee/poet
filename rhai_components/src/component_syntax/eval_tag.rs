use rhai::EvalAltResult;
use rhai::EvalContext;

use super::attribute_value::AttributeValue;
use super::expression_collection::ExpressionCollection;
use super::tag::Tag;
use crate::escape_html_attribute::escape_html_attribute;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

pub fn eval_tag(
    eval_context: &mut EvalContext,
    expression_collection: &mut ExpressionCollection,
    tag: &Tag,
) -> Result<SmartString, Box<EvalAltResult>> {
    let mut result = SmartString::new_const();

    if tag.is_closing {
        result.push_str("</");
        result.push_str(&tag.tag_name.name);
        result.push('>');

        return Ok(result);
    }

    result.push('<');
    result.push_str(&tag.tag_name.name);

    for attribute in &tag.attributes {
        result.push(' ');
        result.push_str(&attribute.name);

        if let Some(value) = &attribute.value {
            result.push('=');
            result.push('"');
            match value {
                AttributeValue::Expression(expression_reference) => {
                    result.push_str(&escape_html_attribute(
                        &expression_collection
                            .eval_expression(eval_context, expression_reference)?
                            .to_string(),
                    ));
                }
                AttributeValue::Text(text) => {
                    result.push_str(text);
                }
            };
            result.push('"');
        }
    }

    if tag.is_self_closing && !tag.tag_name.is_void_element() {
        result.push_str(" />");
    } else {
        result.push('>');
    }

    Ok(result)
}
