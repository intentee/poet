use std::sync::Arc;

use rhai::Array;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Map;

use super::attribute_value::AttributeValue;
use super::component_registry::ComponentRegistry;
use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::tag::Tag;
use super::tag_stack_node::TagStackNode;
use crate::rhai_call_template_function::rhai_call_template_function;

pub fn eval_tag_stack_node(
    component_registry: Arc<ComponentRegistry>,
    eval_context: &mut EvalContext,
    current_node: &TagStackNode,
    expression_collection: &mut ExpressionCollection,
) -> Result<String, Box<EvalAltResult>> {
    match current_node {
        TagStackNode::BodyExpression(expression_reference) => {
            let body_expression_result =
                expression_collection.eval_expression(eval_context, expression_reference)?;

            if body_expression_result.is_array() {
                let body_expression_array: Array = body_expression_result
                    .as_array_ref()
                    .map(|array| array.to_vec())
                    .unwrap_or_default();
                let mut combined_ret = String::new();

                for item in body_expression_array {
                    combined_ret.push_str(&item.to_string());
                }

                Ok(combined_ret)
            } else {
                Ok(body_expression_result.to_string())
            }
        }
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let mut result = String::new();

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
                let closing_tag = Tag {
                    attributes: vec![],
                    is_closing: true,
                    is_self_closing: false,
                    tag_name: opening_tag.tag_name.clone(),
                };

                result.push_str(
                    &eval_tag(eval_context, expression_collection, &closing_tag)
                        .unwrap_or_default(),
                );

                return Ok(result);
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.tag_name.is_component()
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

                let context = match eval_context.scope().get("context") {
                    Some(context) => context.clone(),
                    None => {
                        return Err(EvalAltResult::ErrorRuntime(
                            "'context' variable not found in scope".into(),
                            rhai::Position::NONE,
                        )
                        .into());
                    }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use rhai::CustomType;
    use rhai::Engine;
    use rhai::Func;
    use rhai::TypeBuilder;
    use rhai::module_resolvers::FileModuleResolver;

    use super::ComponentRegistry;
    use crate::builds_engine::BuildsEngine;
    use crate::component_syntax::component_reference::ComponentReference;

    fn fixtures_path() -> String {
        format!("{}/src/component_syntax/fixtures", env!("CARGO_MANIFEST_DIR"))
    }

    #[derive(Clone, Default)]
    struct DummyContext;

    impl CustomType for DummyContext {
        fn build(_builder: TypeBuilder<Self>) {}
    }

    struct LocalBuilder {
        registry: Arc<ComponentRegistry>,
    }

    impl BuildsEngine for LocalBuilder {
        fn component_registry(&self) -> Arc<ComponentRegistry> {
            self.registry.clone()
        }

        fn prepare_engine(&self, engine: &mut Engine) -> Result<()> {
            engine.set_module_resolver(FileModuleResolver::new_with_path(fixtures_path()));
            engine.build_type::<DummyContext>();

            Ok(())
        }
    }

    fn registry_with(names: &[&str]) -> Arc<ComponentRegistry> {
        let registry = Arc::new(ComponentRegistry::default());

        for name in names {
            registry.register_component(ComponentReference {
                name: (*name).to_string(),
                path: (*name).to_string(),
            });
        }

        registry
    }

    fn render_with(component_names: &[&str], script: &str) -> Result<String> {
        let builder = LocalBuilder {
            registry: registry_with(component_names),
        };

        builder.create_engine().and_then(|engine| {
            Func::<(DummyContext,), String>::create_from_script(engine, script, "template")
                .map_err(anyhow::Error::from)
                .and_then(|renderer| renderer(DummyContext).map_err(anyhow::Error::from))
        })
    }

    #[test]
    fn returns_error_when_context_variable_missing_from_scope_for_component_tag() -> Result<()> {
        let builder = LocalBuilder {
            registry: registry_with(&[]),
        };

        assert!(builder.create_engine().is_ok_and(|engine| {
            engine
                .eval::<String>(r#"component { <Note /> }"#)
                .is_err_and(|error| error.to_string().contains("'context'"))
        }));

        Ok(())
    }

    #[test]
    fn returns_error_when_component_template_function_fails() -> Result<()> {
        let result = render_with(
            &["ThrowingComponent"],
            r#"
                fn template(context) {
                    component { <ThrowingComponent /> }
                }
            "#,
        );

        assert!(result
            .is_err_and(|error| error.to_string().contains("Failed to call component function")));

        Ok(())
    }

    #[test]
    fn renders_component_with_expression_attribute() -> Result<()> {
        assert!(render_with(
            &["Note"],
            r#"
                fn template(context) {
                    component { <Note type={"warn"}>hi</Note> }
                }
            "#,
        )
        .is_ok_and(|rendered| rendered.contains("note--warn") && rendered.contains("hi")));

        Ok(())
    }

    #[test]
    fn renders_component_with_no_value_attribute() -> Result<()> {
        assert!(render_with(
            &["Bare"],
            r#"
                fn template(context) {
                    component { <Bare disabled>hi</Bare> }
                }
            "#,
        )
        .is_ok_and(|rendered| {
            rendered.contains("data-disabled=\"yes\"") && rendered.contains("hi")
        }));

        Ok(())
    }

    #[test]
    fn renders_array_body_expression_by_concatenating_items() -> Result<()> {
        assert!(render_with(
            &[],
            r#"
                fn template(context) {
                    component { <div>{["a", "b", "c"]}</div> }
                }
            "#,
        )
        .is_ok_and(|rendered| rendered.contains("<div>abc</div>")));

        Ok(())
    }

    #[test]
    fn returns_error_when_body_expression_evaluation_fails() -> Result<()> {
        assert!(render_with(
            &[],
            r#"
                fn template(context) {
                    component { <div>{nonexistent_variable}</div> }
                }
            "#,
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn returns_error_when_attribute_expression_evaluation_fails() -> Result<()> {
        assert!(render_with(
            &[],
            r#"
                fn template(context) {
                    component { <div data-x={nonexistent_variable}>hi</div> }
                }
            "#,
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn returns_error_when_component_attribute_expression_evaluation_fails() -> Result<()> {
        assert!(render_with(
            &["Bare"],
            r#"
                fn template(context) {
                    component { <Bare data-x={nonexistent_variable}>hi</Bare> }
                }
            "#,
        )
        .is_err());

        Ok(())
    }
}
