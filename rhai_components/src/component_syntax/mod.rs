mod attribute;
mod attribute_value;
mod combine_output_symbols;
mod combine_tag_stack;
pub mod component_reference;
pub mod component_registry;
mod eval_tag;
mod eval_tag_stack_node;
pub mod evaluator_factory;
mod expression_collection;
mod expression_reference;
mod output_combined_symbol;
mod output_semantic_symbol;
mod output_symbol;
pub mod parse_component;
mod parser_state;
mod tag;
pub mod tag_name;
pub mod tag_stack_node;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use dashmap::DashSet;
    use rhai::CustomType;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::Func;
    use rhai::Map;
    use rhai::TypeBuilder;
    use rhai::module_resolvers::FileModuleResolver;

    use super::component_reference::ComponentReference;
    use super::component_registry::ComponentRegistry;
    use super::evaluator_factory::EvaluatorFactory;
    use super::parse_component::parse_component;

    #[derive(Clone, Default)]
    struct DummyAssetCollection {
        assets: Arc<DashSet<String>>,
    }

    impl DummyAssetCollection {
        fn rhai_add(&mut self, asset: String) {
            self.assets.insert(asset);
        }
    }

    impl CustomType for DummyAssetCollection {
        fn build(mut builder: TypeBuilder<Self>) {
            builder
                .with_name("DummyAssetCollection")
                .with_fn("add", Self::rhai_add);
        }
    }

    #[derive(Clone, Default)]
    struct DummyContext {
        assets: DummyAssetCollection,
    }

    impl DummyContext {
        fn rhai_assets(&mut self) -> DummyAssetCollection {
            self.assets.clone()
        }
    }

    impl CustomType for DummyContext {
        fn build(mut builder: TypeBuilder<Self>) {
            builder
                .with_name("DummyContext")
                .with_get("assets", Self::rhai_assets);
        }
    }

    fn build_minimal_engine() -> Engine {
        let component_registry = Arc::new(ComponentRegistry::default());
        let evaluator_factory = EvaluatorFactory { component_registry };
        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
        engine.set_module_resolver(FileModuleResolver::new_with_path(format!(
            "{}/src/component_syntax/fixtures",
            env!("CARGO_MANIFEST_DIR")
        )));
        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        engine
    }

    #[test]
    fn renders_full_document_with_components_attributes_and_body_expressions() -> Result<()> {
        let component_context = DummyContext::default();
        let component_registry = Arc::new(ComponentRegistry::default());

        component_registry.register_component(ComponentReference {
            name: "LayoutHomepage".to_string(),
            path: "LayoutHomepage".to_string(),
        });

        component_registry.register_component(ComponentReference {
            name: "Note".to_string(),
            path: "Note".to_string(),
        });

        let evaluator_factory = EvaluatorFactory {
            component_registry: component_registry.clone(),
        };

        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
        engine.set_module_resolver(FileModuleResolver::new_with_path(format!(
            "{}/src/component_syntax/fixtures",
            env!("CARGO_MANIFEST_DIR")
        )));

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        engine.build_type::<DummyAssetCollection>();
        engine.build_type::<DummyContext>();

        let mut props = Map::new();

        props.insert("bar".into(), "baz".into());

        let props_dynamic = Dynamic::from_map(props);
        let content_dynamic = Dynamic::from("");
        let context = component_context.clone();
        let rendered_matches =
            Func::<(DummyContext, Dynamic, Dynamic), String>::create_from_script(
                engine,
                r#"
                    import "LayoutHomepage" as LayoutHomepage;
                    import "Note" as Note;

                    fn template(context, props, content) {
                        context.assets.add("resouces/controller_foo.tsx");

                        component {
                            <!DOCTYPE html>
                            <LayoutHomepage extraBodyClass="my-extra-class">
                                < button
                                    class="myclass"
                                    data-foo={props.bar}
                                    data-fooz={`${props.bar}`}
                                    data-gooz={if true {
                                        component {
                                            <div />
                                        }
                                    } else {
                                        ":)"
                                    }}
                                    disabled
                                >
                                    <b><i><u>test</u></i></b>
                                    Hello! :D
                                    {" - "}
                                    <br />

                                    <Note type="warn">
                                        {if content.is_empty() {
                                            component {
                                                <div>
                                                    NOTE EMPTY CONTENT
                                                </div>
                                            }
                                        } else {
                                            content
                                        }}
                                    </Note>
                                </button>
                            </LayoutHomepage>
                        }
                    }
                "#,
                "template",
            )
            .is_ok_and(|renderer| {
                renderer(context, props_dynamic, content_dynamic).is_ok_and(|rendered| {
                    rendered.contains("<!DOCTYPE html>")
                        && rendered.contains("<html lang=\"en\">")
                        && rendered.contains("<title>Poet</title>")
                        && rendered.contains("<button")
                        && rendered.contains("class=\"myclass\"")
                        && rendered.contains("data-foo=\"baz\"")
                        && rendered.contains("data-fooz=\"baz\"")
                        && rendered.contains("disabled")
                        && rendered.contains("<b><i><u>test</u></i></b>")
                        && rendered.contains("Hello! :D")
                        && rendered.contains(" - ")
                        && rendered.contains("<br>")
                        && rendered.contains("class=\"note note--warn\"")
                        && rendered.contains("NOTE EMPTY CONTENT")
                        && rendered.contains("</button>")
                        && rendered.contains("</body>")
                        && rendered.contains("</html>")
                })
            });

        assert!(rendered_matches);
        assert!(
            component_context
                .assets
                .assets
                .contains("resouces/controller_foo.tsx")
        );

        Ok(())
    }

    #[test]
    fn parses_mismatched_closing_tag_as_parse_error() -> Result<()> {
        let engine = build_minimal_engine();

        assert!(
            engine
                .compile(r#"component { <div></span> }"#)
                .is_err_and(|error| error.to_string().contains("Mismatched closing tag"))
        );

        Ok(())
    }

    #[test]
    fn parses_self_close_after_attribute_name_renders_self_closing_tag() -> Result<()> {
        let engine = build_minimal_engine();

        assert!(
            engine
                .eval::<String>(r#"component { <input checked/> }"#)
                .is_ok_and(|rendered| rendered.contains("<input checked"))
        );

        Ok(())
    }
}
