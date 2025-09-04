mod attribute;
mod attribute_value;
mod combine_output_symbols;
mod combine_tag_stack;
pub mod component_meta_module;
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

    use super::component_meta_module::ComponentMetaModule;
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

    #[tokio::test]
    async fn test_docs_parser() -> Result<()> {
        let component_context = DummyContext::default();
        let component_registry = Arc::new(ComponentRegistry::default());

        component_registry.register_component(ComponentReference {
            global_fn_name: "LayoutHomepage_123".to_string(),
            name: "LayoutHomepage".to_string(),
            path: "LayoutHomepage".to_string(),
        });

        component_registry.register_component(ComponentReference {
            global_fn_name: "Note_123".to_string(),
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
            "{}/src/rhai_components/fixtures",
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

        let meta_module =
            ComponentMetaModule::from(component_registry.clone()).into_global_module(&engine)?;

        engine.register_global_module(meta_module.into());

        let renderer = Func::<(DummyContext, Dynamic, Dynamic), String>::create_from_script(
            engine,
            r#"
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
        )?;

        println!(
            "{}",
            renderer(
                component_context.clone(),
                Dynamic::from_map({
                    let mut props = Map::new();

                    props.insert("bar".into(), "baz".into());

                    props
                }),
                Dynamic::from(""),
            )?
        );

        assert!(
            component_context
                .assets
                .assets
                .contains("resouces/controller_foo.tsx")
        );
        // assert!(false);

        Ok(())
    }
}
