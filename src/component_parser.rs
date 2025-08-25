// TODO: this file is a draft; once its done, enable warnings again
#![allow(warnings)]
#![allow(clippy::all)]

#[derive(Clone, Debug)]
enum OutputSymbol {
    BodyExpression,
    Text(String),
    TagContent(String),
    TagExpression,
}

#[repr(i32)]
enum ParserState {
    Start = 0,
    OpeningBracket = 1,
    Body = 2,
    BodyExpression = 3,
    TagContent = 4,
    TagExpression = 5,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::OpeningBracket),
            2 => Ok(ParserState::Body),
            3 => Ok(ParserState::BodyExpression),
            4 => Ok(ParserState::TagContent),
            5 => Ok(ParserState::TagExpression),
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
                //     "Symbols: {:?}, tag: {:?}",
                //     symbols,
                //     state.tag()
                // );
                let last_symbol = symbols.last().unwrap().as_str();

                let push_to_state =
                    |state: &mut Dynamic, value: OutputSymbol| match state.as_array_mut() {
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
                            state.set_tag(ParserState::OpeningBracket as i32);

                            Ok(Some("{".into()))
                        }
                        ParserState::OpeningBracket => {
                            state.set_tag(ParserState::Body as i32);

                            Ok(Some("$raw$".into()))
                        }
                        ParserState::Body => match last_symbol {
                            "{" => {
                                state.set_tag(ParserState::BodyExpression as i32);

                                Ok(Some("$inner$".into()))
                            }
                            "}" => Ok(None),
                            "<" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => {
                                push_to_state(state, OutputSymbol::Text(last_symbol.to_string()))?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::BodyExpression => match last_symbol {
                            "$inner$" => {
                                push_to_state(state, OutputSymbol::BodyExpression)?;

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
                        ParserState::TagContent => match last_symbol {
                            ">" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                            "{" => {
                                state.set_tag(ParserState::TagExpression as i32);

                                Ok(Some("$inner$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagExpression => match last_symbol {
                            "$inner$" => {
                                push_to_state(state, OutputSymbol::TagExpression)?;

                                state.set_tag(ParserState::TagContent as i32);

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

                // println!("Inputs: {:#?}, tag: {:?}", inputs, state.tag());
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
                    match node.clone().try_cast::<OutputSymbol>().unwrap() {
                        OutputSymbol::BodyExpression | OutputSymbol::TagExpression => {
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
                        OutputSymbol::TagContent(text) => {
                            result.push_str(format!("[{text}]").as_str());
                        }
                        OutputSymbol::Text(text) => result.push_str(&text),
                    }
                }

                Ok(Dynamic::from(result))
            },
        );

        println!(
            "{:?}",
            engine.eval::<String>(
                r#"
            fn template(assets, content, props) {
                assets.scripts.add("resouces/controller_foo.tsx");

                component {
                    <LayoutHomepage>
                        < div class="myclass" data-foo={props.bar}>
                            Hello! :D
                            {if content.is_empty() {
                                component {
                                    <div>No content</div>
                                }
                            } else {
                                content
                            }}
                        </div>
                    </LayoutHomepage>
                }
            }

            template(#{
                render: || "wow",
                scripts: #{
                    add: |script| script,
                }
            }, "", #{
                bar: "baz"
            })
        "#
            )?
        );

        assert!(false);

        Ok(())
    }
}
