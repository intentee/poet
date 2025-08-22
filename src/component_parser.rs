#[derive(Clone, Debug)]
enum TemplateNode {
    BodyExpressionBlock,
    TagContent(String),
    BodyText(String),
}

#[repr(i32)]
enum ParserState {
    Start = 0,
    ComponentName = 1,
    ParamsOpeningBracket = 2,
    EmptyParams = 3,
    ParamsArgumentIdent = 4,
    ParamsArgumentSeparator = 5,
    ParamsClosingBracket = 6,
    BodyOpeningBracket = 7,
    TagOpeningBracket = 8,
    TagContent = 9,
    TagClosingBracket = 10,
    // BodyOpeningTagSelfClose = 11,
    BodyText = 12,
    BodyExpressionBlock = 13,
    BodyClosingBracket = 14,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::ComponentName),
            2 => Ok(ParserState::ParamsOpeningBracket),
            3 => Ok(ParserState::EmptyParams),
            4 => Ok(ParserState::ParamsArgumentIdent),
            5 => Ok(ParserState::ParamsArgumentSeparator),
            6 => Ok(ParserState::ParamsClosingBracket),
            7 => Ok(ParserState::BodyOpeningBracket),
            8 => Ok(ParserState::TagOpeningBracket),
            9 => Ok(ParserState::TagContent),
            10 => Ok(ParserState::TagClosingBracket),
            // 11 => Ok(ParserState::BodyOpeningTagSelfClose),
            12 => Ok(ParserState::BodyText),
            13 => Ok(ParserState::BodyExpressionBlock),
            14 => Ok(ParserState::BodyClosingBracket),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::EvalAltResult;
    use rhai::EvalContext;
    use rhai::ImmutableString;
    use rhai::LexError;
    use rhai::ParseErrorType;
    use rhai::Position;

    use super::*;

    #[test]
    fn test_docs_parser() -> Result<()> {
        let mut engine = Engine::new();

        engine.register_custom_syntax_with_state_raw(
            // The leading symbol - which needs not be an identifier.
            "component",
            // The custom parser implementation - always returns the next symbol expected
            // 'look_ahead' is the next symbol about to be read
            //
            // Return symbols starting with '$$' also terminate parsing but allows us
            // to determine which syntax variant was actually parsed so we can perform the
            // appropriate action.  This is a convenient short-cut to keeping the value
            // inside the state.
            //
            // The return type is 'Option<ImmutableString>' to allow common text strings
            // to be interned and shared easily, reducing allocations during parsing.
            |symbols, look_ahead, state| {
                println!(
                    "Symbols: {:?}, look_ahead: {:?}, state: {:?}, tag: {:?}",
                    symbols,
                    look_ahead,
                    state,
                    state.tag()
                );

                let push_to_state =
                    |state: &mut Dynamic, value: TemplateNode| match state.as_array_mut() {
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
                            state.set_tag(ParserState::ComponentName as i32);

                            Ok(Some("$ident$".into()))
                        }
                        ParserState::ComponentName => match look_ahead {
                            "(" => {
                                state.set_tag(ParserState::ParamsOpeningBracket as i32);

                                Ok(Some("(".into()))
                            }
                            "()" => {
                                state.set_tag(ParserState::EmptyParams as i32);

                                Ok(Some("()".into()))
                            }
                            _ => Err(LexError::ImproperSymbol(
                                look_ahead.to_string(),
                                format!("Expected '(' after component name, found: {look_ahead}"),
                            )
                            .into_err(Position::NONE)),
                        },
                        ParserState::ParamsOpeningBracket => match look_ahead {
                            ")" => {
                                state.set_tag(ParserState::ParamsClosingBracket as i32);

                                Ok(Some(")".into()))
                            }
                            _ => {
                                state.set_tag(ParserState::ParamsArgumentIdent as i32);

                                Ok(Some("$ident$".into()))
                            }
                        },
                        ParserState::EmptyParams => {
                            state.set_tag(ParserState::BodyOpeningBracket as i32);

                            Ok(Some("{".into()))
                        }
                        ParserState::ParamsArgumentIdent => match look_ahead {
                            "," => {
                                state.set_tag(ParserState::ParamsArgumentSeparator as i32);

                                Ok(Some(",".into()))
                            }
                            ")" => {
                                state.set_tag(ParserState::ParamsClosingBracket as i32);

                                Ok(Some(")".into()))
                            }
                            _ => Err(LexError::ImproperSymbol(
                                look_ahead.to_string(),
                                format!(
                                    "Expected ',' or ')' after parameter name, found: {look_ahead}"
                                ),
                            )
                            .into_err(Position::NONE)),
                        },
                        ParserState::ParamsArgumentSeparator => match look_ahead {
                            ")" => {
                                state.set_tag(ParserState::ParamsClosingBracket as i32);

                                Ok(Some(")".into()))
                            }
                            _ => {
                                state.set_tag(ParserState::ParamsArgumentIdent as i32);

                                Ok(Some("$ident$".into()))
                            }
                        },
                        ParserState::ParamsClosingBracket => {
                            state.set_tag(ParserState::BodyOpeningBracket as i32);

                            Ok(Some("{".into()))
                        }
                        ParserState::BodyOpeningBracket => match look_ahead {
                            "}" => {
                                state.set_tag(ParserState::BodyClosingBracket as i32);

                                Ok(Some("}".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    TemplateNode::BodyText(look_ahead.to_string()),
                                )?;
                                state.set_tag(ParserState::BodyText as i32);

                                Ok(Some(look_ahead.into()))
                            }
                        },
                        ParserState::BodyText
                        | ParserState::BodyExpressionBlock
                        | ParserState::TagClosingBracket => match look_ahead {
                            "<" => {
                                push_to_state(
                                    state,
                                    TemplateNode::TagContent(look_ahead.to_string()),
                                )?;
                                state.set_tag(ParserState::TagOpeningBracket as i32);

                                Ok(Some(look_ahead.into()))
                            }
                            "{" => {
                                push_to_state(state, TemplateNode::BodyExpressionBlock)?;
                                state.set_tag(ParserState::BodyExpressionBlock as i32);

                                Ok(Some("$block$".into()))
                            }
                            "}" => {
                                state.set_tag(ParserState::BodyClosingBracket as i32);

                                Ok(Some("}".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    TemplateNode::BodyText(look_ahead.to_string()),
                                )?;
                                state.set_tag(ParserState::BodyText as i32);

                                Ok(Some(look_ahead.into()))
                            }
                        },
                        ParserState::TagOpeningBracket => {
                            push_to_state(state, TemplateNode::TagContent(look_ahead.to_string()))?;

                            match look_ahead {
                                ">" => {
                                    state.set_tag(ParserState::TagClosingBracket as i32);

                                    Ok(Some(look_ahead.into()))
                                }
                                _ => {
                                    state.set_tag(ParserState::TagContent as i32);

                                    Ok(Some(look_ahead.into()))
                                }
                            }
                        }
                        ParserState::TagContent => match look_ahead {
                            ">" => {
                                push_to_state(
                                    state,
                                    TemplateNode::TagContent(look_ahead.to_string()),
                                )?;
                                state.set_tag(ParserState::TagClosingBracket as i32);

                                Ok(Some(look_ahead.into()))
                            }
                            "{" => {
                                push_to_state(state, TemplateNode::BodyExpressionBlock)?;
                                state.set_tag(ParserState::BodyExpressionBlock as i32);

                                Ok(Some("$block$".into()))
                            }
                            _ => {
                                push_to_state(
                                    state,
                                    TemplateNode::TagContent(look_ahead.to_string()),
                                )?;
                                state.set_tag(ParserState::TagContent as i32);

                                Ok(Some(look_ahead.into()))
                            }
                        },
                        ParserState::BodyClosingBracket => Ok(None),
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
                // let cmd = inputs.last().unwrap().get_string_value().unwrap();
                println!(
                    "Inputs: {:#?}, state: {:#?}, tag: {:?}",
                    inputs,
                    state,
                    state.tag()
                );

                for node in state.as_array_ref()?.iter() {
                    match node.clone().try_cast::<TemplateNode>().unwrap() {
                        TemplateNode::BodyExpressionBlock => {
                            println!("  Expression Block");
                        }
                        TemplateNode::TagContent(content) => {
                            println!("  Tag Content: {content}");
                        }
                        TemplateNode::BodyText(text) => {
                            println!("  Text: {text}");
                        }
                    }
                }

                Err(Box::new(EvalAltResult::ErrorParsing(
                    ParseErrorType::BadInput(LexError::UnexpectedInput(format!(
                        "Unexpected command result"
                    ))),
                    Position::NONE,
                )))
            },
        );

        // println!("{:?}", engine.eval::<String>(r#"
        //     component Nothing() {}
        //
        //     component Admonition(content, type) {
        //     }
        //
        //     component Admonition2(content, type) {
        //         Hellow worldz!
        //         <div>xd</div>
        //     }
        // "#)?);
        println!(
            "{:?}",
            engine.eval::<String>(
                r#"
            component Admonition2(content, type) {
                Hellow worldz!
                Hello world !
                Foo bar.
                Foo bar .
                http://example.com
                <div alt={type}>
                  Foo
                  {content}
                  {if path == "xd" { "wow" } else { ":(" }}
                  Bar
                </div>
            }
        "#
            )?
        );

        assert!(false);

        Ok(())
    }

    #[test]
    fn test_component_parser() -> Result<()> {
        let mut engine = Engine::new();

        engine.register_custom_syntax_with_state_raw(
            // The leading symbol - which needs not be an identifier.
            "perform",
            // The custom parser implementation - always returns the next symbol expected
            // 'look_ahead' is the next symbol about to be read
            //
            // Return symbols starting with '$$' also terminate parsing but allows us
            // to determine which syntax variant was actually parsed so we can perform the
            // appropriate action.  This is a convenient short-cut to keeping the value
            // inside the state.
            //
            // The return type is 'Option<ImmutableString>' to allow common text strings
            // to be interned and shared easily, reducing allocations during parsing.
            |symbols, look_ahead, state| match symbols.len() {
                // perform ...
                1 => Ok(Some("$ident$".into())),
                // perform command ...
                2 => match symbols[1].as_str() {
                    "action" => Ok(Some("$expr$".into())),
                    "hello" => Ok(Some("world".into())),
                    "update" | "check" | "add" | "remove" => Ok(Some("$ident$".into())),
                    "cleanup" => Ok(Some("$$cleanup".into())),
                    cmd => Err(LexError::ImproperSymbol(
                        symbols[1].to_string(),
                        format!("Improper command: {cmd}"),
                    )
                    .into_err(Position::NONE)),
                },
                // perform command arg ...
                3 => match (symbols[1].as_str(), symbols[2].as_str()) {
                    ("action", _) => Ok(Some("$$action".into())),
                    ("hello", "world") => Ok(Some("$$hello-world".into())),
                    ("update", arg) => match arg {
                        "system" => Ok(Some("$$update-system".into())),
                        "client" => Ok(Some("$$update-client".into())),
                        _ => Err(LexError::ImproperSymbol(
                            symbols[1].to_string(),
                            format!("Cannot update {arg}"),
                        )
                        .into_err(Position::NONE)),
                    },
                    ("check", arg) => Ok(Some("$$check".into())),
                    ("add", arg) => Ok(Some("$$add".into())),
                    ("remove", arg) => Ok(Some("$$remove".into())),
                    (cmd, arg) => Err(LexError::ImproperSymbol(
                        symbols[2].to_string(),
                        format!("Invalid argument for command {cmd}: {arg}"),
                    )
                    .into_err(Position::NONE)),
                },
                _ => unreachable!(),
            },
            // No variables declared/removed by this custom syntax
            false,
            // Implementation function
            |context, inputs, state| {
                let cmd = inputs.last().unwrap().get_string_value().unwrap();

                match cmd {
                    "$$cleanup" => Ok(Dynamic::from("cleanup")),
                    "$$action" => Ok(Dynamic::from("action")),
                    "$$hello-world" => Ok(Dynamic::from("hello-world")),
                    "$$update-system" => Ok(Dynamic::from("update-system")),
                    "$$update-client" => Ok(Dynamic::from("update-client")),
                    "$$check" => Ok(Dynamic::from("check")),
                    "$$add" => Ok(Dynamic::from("add")),
                    "$$remove" => Ok(Dynamic::from("remove")),
                    _ => Err(Box::new(EvalAltResult::ErrorParsing(
                        ParseErrorType::BadInput(LexError::UnexpectedInput(format!(
                            "Unexpected command result: {cmd}"
                        ))),
                        Position::NONE,
                    ))),
                }
            },
        );

        // let result = engine.eval::<i64>("inc(41)")?;
        println!("{:?}", engine.eval::<String>(r#"perform hello world;"#)?);

        assert!(false);

        Ok(())
    }
}
