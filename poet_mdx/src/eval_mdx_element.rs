use markdown::mdast::AttributeContent;
use markdown::mdast::AttributeValue;
use markdown::mdast::AttributeValueExpression;
use markdown::mdast::MdxJsxAttribute;
use markdown::mdast::Node;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::Map;
use rhai_components::component_syntax::tag_name::TagName;
use rhai_components::escape_html_attribute::escape_html_attribute;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::mdx_error::MdxError;

fn evaluate_attribute<TComponentContext: CustomType>(
    component_context: &TComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    tag_name: &TagName,
    MdxJsxAttribute { name, value }: &MdxJsxAttribute,
) -> Result<Dynamic, MdxError> {
    match value {
        None => Ok(true.into()),
        Some(AttributeValue::Literal(literal)) => Ok(literal.into()),
        Some(AttributeValue::Expression(AttributeValueExpression { value, .. })) => {
            rhai_template_renderer
                .render_expression(component_context.clone(), value)
                .map_err(|source| MdxError::EvaluateAttribute {
                    attribute_name: name.clone(),
                    tag_name: tag_name.name.clone(),
                    source,
                })
        }
    }
}

fn render_html_element(
    children: &[Node],
    evaluated_children: &str,
    props: Map,
    tag_name: &TagName,
) -> String {
    let rendered_props: String = props
        .into_iter()
        .map(|(prop_name, prop_value)| {
            if prop_value.is_bool() {
                format!("{prop_name} ")
            } else {
                format!(
                    "{prop_name}=\"{}\" ",
                    escape_html_attribute(&prop_value.to_string())
                )
            }
        })
        .collect();
    let closing_tag = if !children.is_empty() || !tag_name.is_void_element() {
        tag_name.closing_tag()
    } else {
        String::new()
    };

    format!(
        "<{} {rendered_props}>{evaluated_children}{closing_tag}",
        tag_name.name
    )
}

pub fn eval_mdx_element<TComponentContext: CustomType>(
    attributes: &[AttributeContent],
    children: &[Node],
    component_context: &TComponentContext,
    evaluated_children: String,
    name: Option<&String>,
    rhai_template_renderer: &RhaiTemplateRenderer,
) -> Result<String, MdxError> {
    let tag_name = TagName {
        name: name.ok_or(MdxError::ElementWithoutName)?.clone(),
    };
    let mut props = Map::new();

    for attribute in attributes {
        match attribute {
            AttributeContent::Expression(_) => {
                return Err(MdxError::AttributeExpressionNotSupported {
                    tag_name: tag_name.name,
                });
            }
            AttributeContent::Property(mdx_jsx_attribute) => {
                props.insert(
                    mdx_jsx_attribute.name.as_str().into(),
                    evaluate_attribute(
                        component_context,
                        rhai_template_renderer,
                        &tag_name,
                        mdx_jsx_attribute,
                    )?,
                );
            }
        }
    }

    if tag_name.is_void_element() && !children.is_empty() {
        return Err(MdxError::VoidElementWithChildren {
            tag_name: tag_name.name,
        });
    }

    if tag_name.is_component() {
        rhai_template_renderer
            .render(
                &tag_name.name,
                component_context.clone(),
                Dynamic::from_map(props),
                Dynamic::from(evaluated_children),
            )
            .map_err(|source| MdxError::RenderComponent {
                tag_name: tag_name.name,
                source,
            })
    } else {
        Ok(render_html_element(
            children,
            &evaluated_children,
            props,
            &tag_name,
        ))
    }
}
