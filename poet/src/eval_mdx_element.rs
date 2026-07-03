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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use markdown::mdast::MdxJsxExpressionAttribute;
    use markdown::mdast::Text;
    use rhai::Engine;
    use rhai::TypeBuilder;
    use rhai_components::component_syntax::component_registry::ComponentRegistry;
    use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

    use super::*;

    #[derive(Clone)]
    struct DummyContext;

    impl CustomType for DummyContext {
        fn build(mut builder: TypeBuilder<Self>) {
            builder.with_name("DummyContext");
        }
    }

    fn renderer() -> Result<RhaiTemplateRenderer> {
        RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: Arc::new(ComponentRegistry::default()),
            expression_engine: Engine::new_raw(),
        })
    }

    fn literal_attribute(name: &str, value: &str) -> AttributeContent {
        AttributeContent::Property(MdxJsxAttribute {
            name: name.to_string(),
            value: Some(AttributeValue::Literal(value.to_string())),
        })
    }

    #[test]
    fn renders_non_component_element_with_literal_and_boolean_attributes() -> Result<()> {
        let rendered = eval_mdx_element(
            &[
                literal_attribute("class", "highlight"),
                AttributeContent::Property(MdxJsxAttribute {
                    name: "hidden".to_string(),
                    value: None,
                }),
            ],
            &[],
            &DummyContext,
            String::new(),
            &Some("div".to_string()),
            &renderer()?,
        )?;

        assert!(rendered.contains("<div "));
        assert!(rendered.contains("class=\"highlight\""));
        assert!(rendered.contains("hidden"));
        assert!(rendered.contains("</div>"));

        Ok(())
    }

    #[test]
    fn errors_when_void_element_has_children() -> Result<()> {
        assert!(
            eval_mdx_element(
                &[],
                &[Node::Text(Text {
                    value: "child".to_string(),
                    position: None,
                })],
                &DummyContext,
                "child".to_string(),
                &Some("br".to_string()),
                &renderer()?,
            )
            .is_err()
        );

        Ok(())
    }

    #[test]
    fn errors_on_attribute_expression() -> Result<()> {
        assert!(
            eval_mdx_element(
                &[AttributeContent::Expression(MdxJsxExpressionAttribute {
                    value: "spread".to_string(),
                    stops: Vec::new(),
                })],
                &[],
                &DummyContext,
                String::new(),
                &Some("div".to_string()),
                &renderer()?,
            )
            .is_err()
        );

        Ok(())
    }

    #[test]
    fn errors_when_element_has_no_name() -> Result<()> {
        assert!(
            eval_mdx_element(&[], &[], &DummyContext, String::new(), &None, &renderer()?,).is_err()
        );

        Ok(())
    }

    #[test]
    fn renders_attribute_with_evaluated_expression_value() -> Result<()> {
        let rendered = eval_mdx_element(
            &[AttributeContent::Property(MdxJsxAttribute {
                name: "data-label".to_string(),
                value: Some(AttributeValue::Expression(AttributeValueExpression {
                    value: "\"hi\"".to_string(),
                    stops: Vec::new(),
                })),
            })],
            &[],
            &DummyContext,
            String::new(),
            &Some("div".to_string()),
            &renderer()?,
        )?;

        assert!(rendered.contains("data-label=\"hi\""));

        Ok(())
    }
}
