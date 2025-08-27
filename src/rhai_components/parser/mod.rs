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
    use anyhow::Result;
    use rhai::Engine;

    use super::evaluator_factory::EvaluatorFactory;
    use super::parse_component::parse_component;

    #[test]
    fn test_docs_parser() -> Result<()> {
        let evaluator_factory = EvaluatorFactory {};
        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
        // engine.set_strict_variables(true);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        // engine.register_static_module("LayoutHomepage", engine.module_resolver().resolve(
        //     &engine,
        //     None,
        //     "shortcodes/LayoutHomepage.rhai",
        //     Position::NONE,
        // )?);
        //
        // let note_module = engine.module_resolver().resolve(
        //     &engine,
        //     None,
        //     "shortcodes/Note.rhai",
        //     Position::NONE,
        // )?;
        //
        // for signature in note_module.gen_fn_signatures_with_mapper(|name| format!("Note::{name}").into()) {
        //     println!("Function signature: {:#?}", signature);
        // }
        //
        // println!("Note::template function: {:#?}", note_module.get_script_fn("template", 3));

        // engine.register_static_module("Note", note_module);

        println!(
            "{}",
            engine.eval::<String>(
                r#"
            fn MyComponent(context, content, props) {
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
            }, "", #{
                bar: "baz tag \" attribute"
            })
        "#
            )?
        );

        assert!(false);

        Ok(())
    }
}
