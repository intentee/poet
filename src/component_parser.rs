// TODO: this file is a draft; once its done, enable warnings again
#![allow(warnings)]
#![allow(clippy::all)]

use std::collections::VecDeque;

use anyhow::Result;
use rhai::Dynamic;
use rhai::Engine;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;
use rhai::ImmutableString;
use rhai::LexError;
use rhai::ParseError;
use rhai::ParseErrorType;
use rhai::Position;

#[derive(Clone, Debug)]
enum OutputSymbol {
    BodyExpression,
    Text(String),
    TagLeftAnglePlusWhitespace(String),
    TagCloseBeforeNamePlusWhitespace(String),
    TagName(String),
    TagContent(String),
    TagAttributeName(String),
    TagAttributeValueExpression,
    TagAttributeValueString(String),
    TagSelfClose,
    TagRightAngle,
}

#[derive(Debug)]
enum OutputCombinedSymbol {
    Text(String),
    TagLeftAngle,
    TagCloseBeforeName,
    TagName(String),
    TagAttributeName(String),
    TagAttributeValue(String),
    TagSelfClose,
    TagRightAngle,
}

enum OutputSemanticSymbol {
    Tag {
        attributes: Vec<(String, Option<String>)>,
        name: String,
        is_opening: bool,
        is_self_closing: bool,
    },
    Text(String),
}

#[repr(i32)]
enum ParserState {
    Start = 0,
    OpeningBracket = 1,
    Body = 2,
    BodyExpression = 3,
    TagLeftAnglePlusWhitespace = 4,
    TagCloseBeforeNamePlusWhitespace = 5,
    TagName = 6,
    TagContent = 7,
    TagAttributeName = 8,
    TagAttributeValue = 9,
    TagAttributeValueString = 10,
    TagSelfClose = 11,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::OpeningBracket),
            2 => Ok(ParserState::Body),
            3 => Ok(ParserState::BodyExpression),
            4 => Ok(ParserState::TagLeftAnglePlusWhitespace),
            5 => Ok(ParserState::TagCloseBeforeNamePlusWhitespace),
            6 => Ok(ParserState::TagName),
            7 => Ok(ParserState::TagContent),
            8 => Ok(ParserState::TagAttributeName),
            9 => Ok(ParserState::TagAttributeValue),
            10 => Ok(ParserState::TagAttributeValueString),
            11 => Ok(ParserState::TagSelfClose),
            _ => Err(()),
        }
    }
}

/// Taken from Tera: https://github.com/Keats/tera/blob/master/src/utils.rs
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        match c {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            _ => output.push(c),
        }
    }

    output
}

pub fn parse_component(
    symbols: &[ImmutableString],
    state: &mut Dynamic,
) -> core::result::Result<Option<ImmutableString>, ParseError> {
    // println!("Symbols: {:?}, tag: {:?}", symbols, state.tag());

    let last_symbol = symbols.last().unwrap().as_str();

    let push_to_state = |state: &mut Dynamic, value: OutputSymbol| match state.as_array_mut() {
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
                        OutputSymbol::TagLeftAnglePlusWhitespace(last_symbol.to_string()),
                    )?;
                    state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

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
            ParserState::TagLeftAnglePlusWhitespace => match last_symbol {
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(
                        state,
                        OutputSymbol::TagLeftAnglePlusWhitespace(last_symbol.to_string()),
                    )?;
                    state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                "/" => {
                    push_to_state(
                        state,
                        OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.to_string()),
                    )?;
                    state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()))?;
                    state.set_tag(ParserState::TagName as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagCloseBeforeNamePlusWhitespace => match last_symbol {
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(
                        state,
                        OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.to_string()),
                    )?;
                    state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()))?;
                    state.set_tag(ParserState::TagName as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagName => match last_symbol {
                ">" => {
                    push_to_state(state, OutputSymbol::TagRightAngle)?;
                    state.set_tag(ParserState::Body as i32);

                    Ok(Some("$raw$".into()))
                }
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(state, OutputSymbol::TagContent(last_symbol.to_string()))?;
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()))?;
                    state.set_tag(ParserState::TagName as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagContent => match last_symbol {
                ">" => {
                    push_to_state(state, OutputSymbol::TagRightAngle)?;
                    state.set_tag(ParserState::Body as i32);

                    Ok(Some("$raw$".into()))
                }
                "{" => Err(LexError::ImproperSymbol(
                    symbols.last().unwrap().to_string(),
                    format!(
                        "Invalid expression block start at token: {}",
                        symbols.last().unwrap()
                    ),
                )
                .into_err(Position::NONE)),
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(state, OutputSymbol::TagContent(last_symbol.to_string()))?;
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                "/" => {
                    push_to_state(state, OutputSymbol::TagSelfClose)?;
                    state.set_tag(ParserState::TagSelfClose as i32);

                    Ok(Some(">".into()))
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
                    push_to_state(state, OutputSymbol::TagRightAngle)?;
                    state.set_tag(ParserState::Body as i32);

                    Ok(Some("$raw$".into()))
                }
                "/" => {
                    push_to_state(state, OutputSymbol::TagSelfClose)?;
                    state.set_tag(ParserState::TagSelfClose as i32);

                    Ok(Some(">".into()))
                }
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(state, OutputSymbol::TagContent(last_symbol.to_string()))?;
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
                "$inner$" => {
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                "\"" => {
                    state.set_tag(ParserState::TagAttributeValueString as i32);

                    Ok(Some("$raw$".into()))
                }
                "{" => {
                    push_to_state(state, OutputSymbol::TagAttributeValueExpression)?;
                    state.set_tag(ParserState::TagAttributeValue as i32);

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
            ParserState::TagAttributeValueString => match last_symbol {
                "\"" => {
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(
                        state,
                        OutputSymbol::TagAttributeValueString(last_symbol.to_string()),
                    )?;
                    state.set_tag(ParserState::TagAttributeValueString as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagSelfClose => match last_symbol {
                ">" => {
                    push_to_state(state, OutputSymbol::TagRightAngle)?;
                    state.set_tag(ParserState::Body as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => Err(LexError::ImproperSymbol(
                    symbols.last().unwrap().to_string(),
                    format!(
                        "Invalid self-closing tag end at token: {}",
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
}

pub fn eval_component(
    context: &mut EvalContext,
    inputs: &[Expression],
    state: &Dynamic,
) -> core::result::Result<Dynamic, Box<EvalAltResult>> {
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

    let mut pop_expression_tree = || {
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

    let mut combined_symbols: Vec<OutputCombinedSymbol> = vec![];

    for node in state.as_array_ref()?.iter() {
        match node.clone().try_cast::<OutputSymbol>().unwrap() {
            OutputSymbol::BodyExpression => {
                let chunk = pop_expression_tree()?;

                match combined_symbols.last_mut() {
                    Some(OutputCombinedSymbol::Text(text)) => {
                        text.push_str(&chunk);
                    }
                    _ => {
                        combined_symbols.push(OutputCombinedSymbol::Text(chunk));
                    }
                }
            }
            OutputSymbol::TagAttributeValueExpression => {
                let chunk = pop_expression_tree()?;

                match combined_symbols.last_mut() {
                    Some(OutputCombinedSymbol::TagAttributeValue(value)) => {
                        value.push_str(&chunk);
                    }
                    _ => {
                        combined_symbols.push(OutputCombinedSymbol::TagAttributeValue(chunk));
                    }
                }
            }
            OutputSymbol::TagLeftAnglePlusWhitespace(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagLeftAngle) => {}
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagLeftAngle);
                }
            },
            OutputSymbol::TagCloseBeforeNamePlusWhitespace(_text) => {
                match combined_symbols.last_mut() {
                    Some(OutputCombinedSymbol::TagCloseBeforeName) => {}
                    _ => {
                        combined_symbols.push(OutputCombinedSymbol::TagCloseBeforeName);
                    }
                }
            }
            OutputSymbol::TagContent(text) => {}
            OutputSymbol::TagAttributeValueString(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagAttributeValue(value)) => {
                    value.push_str(&text);
                }
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagAttributeValue(text));
                }
            },
            OutputSymbol::TagAttributeName(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagAttributeName(existing_text)) => {
                    existing_text.push_str(&text);
                }
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagAttributeName(text));
                }
            },
            OutputSymbol::TagName(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagName(existing_text)) => {
                    existing_text.push_str(&text);
                }
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagName(text));
                }
            },
            OutputSymbol::TagSelfClose => {
                combined_symbols.push(OutputCombinedSymbol::TagSelfClose);
            }
            OutputSymbol::TagRightAngle => {
                combined_symbols.push(OutputCombinedSymbol::TagRightAngle);
            }
            OutputSymbol::Text(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::Text(existing_text)) => {
                    existing_text.push_str(&text);
                }
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::Text(text));
                }
            },
        }
    }

    for output_combined_symbol in combined_symbols {
        match output_combined_symbol {
            OutputCombinedSymbol::Text(text) => {
                println!("Text: [{text}]");
            }
            OutputCombinedSymbol::TagLeftAngle => {
                println!("TagLeftAngle");
            }
            OutputCombinedSymbol::TagCloseBeforeName => {
                println!("TagCloseBeforeName");
            }
            OutputCombinedSymbol::TagName(name) => {
                println!("TagName: {name}");
            }
            OutputCombinedSymbol::TagAttributeName(name) => {
                println!("TagAttributeName: {name}");
            }
            OutputCombinedSymbol::TagAttributeValue(value) => {
                println!("TagAttributeValue: {value}");
            }
            OutputCombinedSymbol::TagSelfClose => {
                println!("TagSelfClose");
            }
            OutputCombinedSymbol::TagRightAngle => {
                println!("TagRightAngle");
            }
        }
    }

    Ok(Dynamic::from(String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_parser() -> Result<()> {
        let mut engine = Engine::new();

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            eval_component,
        );

        println!(
            "{:?}",
            engine.eval::<String>(
                r#"
            fn template(assets, content, props) {
                assets.scripts.add("resouces/controller_foo.tsx");

                component {
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
                bar: "baz tag \" attribute"
            })
        "#
            )?
        );

        assert!(false);

        Ok(())
    }
}
