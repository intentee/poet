use rhai::EvalAltResult;
use rhai::EvalContext;

use super::attribute_value::AttributeValue;
use super::escape_html::escape_html;
use super::expression_collection::ExpressionCollection;
use super::tag::Tag;

pub fn eval_tag(
    eval_context: &mut EvalContext,
    expression_collection: &mut ExpressionCollection,
    tag: &Tag,
) -> Result<String, Box<EvalAltResult>> {
    let mut result = String::new();

    if tag.is_closing {
        result.push_str("</");
        result.push_str(&tag.name);
        result.push('>');

        return Ok(result);
    }

    result.push('<');
    result.push_str(&tag.name);

    for attribute in &tag.attributes {
        result.push(' ');
        result.push_str(&attribute.name);

        if let Some(value) = &attribute.value {
            result.push('=');
            result.push('"');
            match value {
                AttributeValue::Expression(expression_reference) => {
                    result.push_str(&escape_html(
                        &expression_collection
                            .eval_expression(eval_context, expression_reference)?
                            .into_string()?,
                    ));
                }
                AttributeValue::Text(text) => {
                    result.push_str(text);
                }
            };
            result.push('"');
        }
    }

    // if tag.is_self_closing {
    //     result.push_str(" />");
    // } else {
    //     result.push('>');
    // }

    result.push('>');

    Ok(result)
}
