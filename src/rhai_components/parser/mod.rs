mod attribute;
mod attribute_value;
mod combine_output_symbols;
mod combine_tag_stack;
mod escape_html;
mod eval_tag;
mod eval_tag_stack_node;
mod evaluator_factory;
mod expression_collection;
mod expression_reference;
mod output_combined_symbol;
mod output_semantic_symbol;
mod output_symbol;
mod parse_component;
mod parser_state;
mod tag;
mod tag_stack_node;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use dashmap::DashSet;
    use rhai::CustomType;
    use rhai::Engine;
    use rhai::Module;
    use rhai::Scope;
    use rhai::TypeBuilder;

    use super::evaluator_factory::EvaluatorFactory;
    use super::parse_component::parse_component;

    #[derive(Clone)]
    struct DummyContext {
        assets: Arc<DashSet<String>>,
    }

    impl DummyContext {
        fn assets(&mut self) -> Arc<DashSet<String>> {
            self.assets.clone()
        }
    }

    impl CustomType for DummyContext {
        fn build(mut builder: TypeBuilder<Self>) {
            builder
                .with_name("DummyContext")
                .with_get("assets", Self::assets);
        }
    }

    impl Default for DummyContext {
        fn default() -> Self {
            Self {
                assets: Arc::new(DashSet::new()),
            }
        }
    }

    #[test]
    fn test_docs_parser() -> Result<()> {
        let component_context = DummyContext::default();
        let evaluator_factory = EvaluatorFactory {};
        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator_with_context(component_context.clone()),
        );

        // engine.build_type::<DummyContext>();

        let meta_module_ast = engine.compile(
            r#"
          // import "LayoutHomepage" as LayoutHomepage;

          fn LayoutHomepage_123(context, props, content) {
            // LayoutHomepage::template(context, props, content)
            `[LAYOUT(${content})]`
          }
        "#,
        )?;

        let module = Module::eval_ast_as_new(Scope::new(), &meta_module_ast, &engine)?;

        engine.register_global_module(module.into());

        println!(
            "{}",
            engine.eval::<String>(
                r#"
            fn MyComponent(context, props, content) {
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

                            <Note>
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

            MyComponent(#{
                render: || "wow",
                assets: #{
                    add: |script| script,
                }
            }, #{
                bar: "baz tag \" attribute"
            }, "")
        "#
            )?
        );

        assert!(false);

        Ok(())
    }
}
