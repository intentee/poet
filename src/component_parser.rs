// TODO: this file is a draft; once its done, enable warnings again
#![allow(warnings)]
#![allow(clippy::all)]

#[derive(Clone, Debug)]
enum OutputSymbol {
    BodyExpression,
    Text(String),
    TagOpening(String),
    TagName(String),
    TagContent(String),
    TagAttributeName(String),
    TagAttributeValueExpression,
    TagContentExpression,
}

#[repr(i32)]
enum ParserState {
    Start = 0,
    OpeningBracket = 1,
    Body = 2,
    BodyExpression = 3,
    TagOpening = 4,
    TagName = 5,
    TagContent = 6,
    TagAttributeName = 7,
    TagAttributeValue = 8,
    TagContentExpression = 9,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::OpeningBracket),
            2 => Ok(ParserState::Body),
            3 => Ok(ParserState::BodyExpression),
            4 => Ok(ParserState::TagOpening),
            5 => Ok(ParserState::TagName),
            6 => Ok(ParserState::TagContent),
            7 => Ok(ParserState::TagAttributeName),
            8 => Ok(ParserState::TagAttributeValue),
            9 => Ok(ParserState::TagContentExpression),
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
                println!("Symbols: {:?}, tag: {:?}", symbols, state.tag());

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
                                    OutputSymbol::TagOpening(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagOpening as i32);

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
                        ParserState::TagOpening => match last_symbol {
                            _ if last_symbol.trim().is_empty() => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagOpening(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagOpening as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagName as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagName => match last_symbol {
                            ">" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ if last_symbol.trim().is_empty() => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagName as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagContent => match last_symbol {
                            "$inner$" => {
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                            ">" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                            "{" => {
                                state.set_tag(ParserState::TagContentExpression as i32);

                                Ok(Some("$inner$".into()))
                            }
                            _ if last_symbol.trim().is_empty() => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagAttributeName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagAttributeName as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagAttributeName => match last_symbol {
                            "=" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagAttributeName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagAttributeValue as i32);

                                Ok(Some("$raw$".into()))
                            }
                            ">" => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::Body as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ if last_symbol.trim().is_empty() => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagContent(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagAttributeName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagAttributeName as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagAttributeValue => match last_symbol {
                            "{" => {
                                push_to_state(state, OutputSymbol::TagAttributeValueExpression)?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$inner$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    OutputSymbol::TagAttributeName(last_symbol.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some("$raw$".into()))
                            }
                        },
                        ParserState::TagContentExpression => match last_symbol {
                            "$inner$" => {
                                push_to_state(state, OutputSymbol::TagContentExpression)?;

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

                let mut pop_expression = || {
                    if let Some(expression) = inputs_deque.pop_front() {
                        Ok(context.eval_expression_tree(expression)?.into_string()?)
                    } else {
                        Err(Box::new(EvalAltResult::ErrorParsing(
                            ParseErrorType::BadInput(LexError::UnexpectedInput(format!(
                                "Exprected expression after component block (got nothing)"
                            ))),
                            Position::NONE,
                        )))
                    }
                };

                for node in state.as_array_ref()?.iter() {
                    match node.clone().try_cast::<OutputSymbol>().unwrap() {
                        OutputSymbol::BodyExpression | OutputSymbol::TagContentExpression => {
                            result.push_str(&pop_expression()?);
                        }
                        OutputSymbol::TagAttributeValueExpression => {
                            result.push_str(&pop_expression()?);
                        }
                        OutputSymbol::TagAttributeName(text)
                        | OutputSymbol::TagContent(text)
                        | OutputSymbol::TagName(text)
                        | OutputSymbol::TagOpening(text)
                        | OutputSymbol::Text(text) => result.push_str(&text),
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
                        < button
                            class="myclass"
                            data-foo={props.bar}
                            data-fooz={`${props.bar}`}
                            disabled
                        >
                            Hello! :D
                            {if content.is_empty() {
                                component {
                                    <div>No content</div>
                                }
                            } else {
                                content
                            }}
                        </button>
                    </LayoutHomepage>
                }
            }

            template(#{
                render: || "wow",
                scripts: #{
                    add: |script| script,
                }
            }, "", #{
                bar: "baz tag attribute"
            })
        "#
            )?
        );

        assert!(false);

        Ok(())
    }
}
