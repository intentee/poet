use anyhow::Result;
use anyhow::anyhow;
use markdown::mdast::AttributeContent;
use markdown::mdast::AttributeValue;
use markdown::mdast::AttributeValueExpression;
use markdown::mdast::MdxJsxAttribute;
use markdown::mdast::Node;
use rhai::CustomType;
use rhai::Dynamic;
use rhai_components::component_syntax::tag_name::TagName;
use rhai_components::escape_html_attribute::escape_html_attribute;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

pub fn eval_mdx_element<TComponentContext>(
    attributes: &[AttributeContent],
    children: &[Node],
    component_context: &TComponentContext,
    evaluated_children: String,
    name: &Option<String>,
    rhai_template_renderer: &RhaiTemplateRenderer,
) -> Result<String>
where
    TComponentContext: CustomType,
{
    let mut result: String = String::new();

    let tag_name = TagName {
        name: name
            .clone()
            .ok_or_else(|| anyhow!("MdxJsxFlowElement without a name"))?,
    };

    let props = {
        let mut props = rhai::Map::new();

        for attribute in attributes {
            match attribute {
                AttributeContent::Expression(_) => {
                    return Err(anyhow!(
                        "Attribute expressions in Markdown are not supported"
                    ));
                }
                AttributeContent::Property(MdxJsxAttribute { name, value }) => {
                    props.insert(
                        name.into(),
                        match value {
                            Some(value) => match value {
                                AttributeValue::Literal(literal) => literal.into(),
                                AttributeValue::Expression(AttributeValueExpression {
                                    value,
                                    ..
                                }) => rhai_template_renderer
                                    .render_expression(component_context.clone(), value)?,
                            },
                            None => true.into(),
                        },
                    );
                }
            }
        }

        props
    };

    if tag_name.is_void_element() && !children.is_empty() {
        return Err(anyhow!("Void element cannot have children"));
    }

    if tag_name.is_component() {
        result.push_str(&rhai_template_renderer.render(
            &tag_name.name,
            component_context.clone(),
            Dynamic::from_map(props),
            Dynamic::from(evaluated_children),
        )?);
    } else {
        result.push_str(&format!("<{} ", tag_name.name));

        for (name, value) in props {
            if value.is_bool() {
                result.push_str(&format!("{name} "));
            } else {
                result.push_str(&format!(
                    "{name}=\"{}\" ",
                    escape_html_attribute(&value.to_string())
                ));
            }
        }

        result.push('>');

        if !children.is_empty() {
            result.push_str(&evaluated_children);
        }

        if !children.is_empty() || !tag_name.is_void_element() {
            result.push_str(&format!("</{}>", tag_name.name));
        }
    }

    Ok(result)
}
