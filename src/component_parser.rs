// TODO: this file is a draft; once its done, enable warnings again
#![allow(warnings)]
#![allow(clippy::all)]

#[derive(Clone, Debug)]
enum OutputNode {
    RhaiBlock,
    Text(String),
}

#[repr(i32)]
enum ParserState {
    Start = 0,
    BodyOpeningBracket = 1,
    Body = 2,
    BodyExpression = 3,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::BodyOpeningBracket),
            2 => Ok(ParserState::Body),
            3 => Ok(ParserState::BodyExpression),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::EvalAltResult;
    use rhai::EvalContext;
    use rhai::Expression;
    use rhai::ImmutableString;
    use rhai::LexError;
    use rhai::ParseErrorType;
    use rhai::Position;

    use super::*;

    #[test]
    fn test_docs_parser() -> Result<()> {
        let mut engine = Engine::new();

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            |symbols, state| {
                // println!(
                //     "Symbols: {:?}, state: {:?}, tag: {:?}",
                //     symbols,
                //     state,
                //     state.tag()
                // );
                let last_symbol = symbols.last().unwrap().as_str();

                let push_to_state =
                    |state: &mut Dynamic, value: OutputNode| match state.as_array_mut() {
                        Ok(mut array) => {
                            array.push(Dynamic::from(value));

                            Ok(())
                        }
                        Err(err) => Err(LexError::ImproperSymbol(
                            symbols.last().unwrap().to_string(),
                            format!(
                                "Invalid state array {err} at token: {}",
                                symbols.last().unwrap()
                            ),
                        )
                        .into_err(Position::NONE)),
                    };

                match ParserState::try_from(state.tag()) {
                    Ok(current_state) => match current_state {
                        ParserState::Start => {
                            *state = Dynamic::from_array(vec![]);
                            state.set_tag(ParserState::BodyOpeningBracket as i32);

                            Ok(Some("{".into()))
                        }
                        ParserState::BodyOpeningBracket => {
                            state.set_tag(ParserState::Body as i32);

                            Ok(Some("$raw$".into()))
                        }
                        ParserState::Body => match last_symbol {
                            "{" => {
                                state.set_tag(ParserState::BodyExpression as i32);

                                Ok(Some("$inner$".into()))
                            }
                            "}" => Ok(None),
                            _ => {
                                push_to_state(state, OutputNode::Text(last_symbol.to_string()))?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::BodyExpression => match last_symbol {
                            "$inner$" => {
                                push_to_state(state, OutputNode::RhaiBlock)?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => Err(LexError::ImproperSymbol(
                                symbols.last().unwrap().to_string(),
                                format!(
                                    "Invalid expression block end at token: {}",
                                    symbols.last().unwrap()
                                ),
                            )
                            .into_err(Position::NONE)),
                        },
                    },
                    Err(_) => {
                        return Err(LexError::ImproperSymbol(
                            symbols.last().unwrap().to_string(),
                            format!("Invalid parser state at token: {}", symbols.last().unwrap()),
                        )
                        .into_err(Position::NONE));
                    }
                }
            },
            // variables can be declared/removed by this custom syntax
            true,
            |context, inputs, state| {
                let mut inputs_deque: VecDeque<&Expression> = inputs.iter().collect();

                // let cmd = inputs.last().unwrap().get_string_value().unwrap();
                println!("Inputs: {:#?}, tag: {:?}", inputs, state.tag());
                //
                // let module = context.engine().module_resolver().resolve(
                //     context.engine(),
                //     None,
                //     "xd.rhai",
                //     Position::NONE,
                // );
                //
                // println!("Module: {:#?}", module);
                let mut result = String::new();

                for node in state.as_array_ref()?.iter() {
                    match node.clone().try_cast::<OutputNode>().unwrap() {
                        OutputNode::RhaiBlock => {
                            if let Some(expression) = inputs_deque.pop_front() {
                                result.push_str(
                                    &context.eval_expression_tree(expression)?.into_string()?,
                                );
                            } else {
                                return Err(Box::new(EvalAltResult::ErrorParsing(
                                    ParseErrorType::BadInput(LexError::UnexpectedInput(format!(
                                        "Exprected expression after component block (got nothing)"
                                    ))),
                                    Position::NONE,
                                )));
                            }
                        }
                        OutputNode::Text(text) => result.push_str(&text),
                    }
                }

                Ok(Dynamic::from(result))
            },
        );

        println!(
            "WHAT CAME OUT: {:?}",
            engine.eval::<String>(
                r#"
            fn template(assets, content, props) {
                component {
                    <!DOCTYPE html>
                    <html lang="en">
                        <head>
                            {assets.render()}
                        </head>
                        <body>
                            {component {
                                <div>nested</div>
                            }}
                            {if content.is_empty() {
                                component {
                                    <div>No content</div>
                                }
                            } else {
                                content
                            }}
                        </body>
                    </html>
                }
            }

            template(#{
                render: || "wow"
            }, "", #{})
        "#
            )?
        );
        // println!("{:?}", engine.eval::<()>(r#"
        //     fn template(assets, content, props) {
        //         assets.styles.add("resources/css/page-docs.css");
        //
        //         component {
        //             <LayoutHomepage>
        //                 <div>
        //                     Hello! :D
        //                     {content}
        //                     {if content.is_empty() {
        //                         "No content"
        //                     } else {
        //                         "Has content"
        //                     }}
        //                     \{content\}
        //                 </div>
        //             </LayoutHomepage>
        //         }
        //     }
        // "#)?);

        assert!(false);

        Ok(())
    }
}
