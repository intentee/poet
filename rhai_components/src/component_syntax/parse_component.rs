use rhai::Dynamic;
use rhai::ImmutableString;
use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::combine_output_symbols::combine_output_symbols;
use super::combine_tag_stack::combine_tag_stack;
use super::output_symbol::OutputSymbol;
use super::parser_state::ParserState;
use super::tag_stack_node::TagStackNode;

pub fn parse_component(
    symbols: &[ImmutableString],
    state: &mut Dynamic,
) -> Result<Option<ImmutableString>, ParseError> {
    let last_symbol = symbols
        .last()
        .ok_or_else(|| LexError::Runtime("No symbols found".to_string()).into_err(Position::NONE))?
        .as_str();

    let push_to_state = |state: &mut Dynamic, value: OutputSymbol| match state.as_array_mut() {
        Ok(mut array) => {
            array.push(Dynamic::from(value));

            Ok(())
        }
        Err(err) => Err(LexError::Runtime(format!(
            "Invalid state array {err} at token: {last_symbol}"
        ))
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
                // This is where the expression ends, so lets optimize the internal state now
                "}" => {
                    let mut semantic_symbols = combine_output_symbols(state)?;

                    let mut tag_stack = TagStackNode::Tag {
                        children: vec![],
                        is_closed: false,
                        opening_tag: None,
                    };

                    combine_tag_stack(
                        &mut tag_stack,
                        &mut Default::default(),
                        &mut semantic_symbols,
                    )?;

                    *state = Dynamic::from(tag_stack);

                    Ok(None)
                }
                "<" => {
                    push_to_state(state, OutputSymbol::TagLeftAnglePlusWhitespace)?;
                    state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::Text(last_symbol.into()))?;
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
                    last_symbol.to_string(),
                    "Invalid expression block end".to_string(),
                )
                .into_err(Position::NONE)),
            },
            ParserState::TagLeftAnglePlusWhitespace => match last_symbol {
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(state, OutputSymbol::TagLeftAnglePlusWhitespace)?;
                    state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                "/" => {
                    push_to_state(
                        state,
                        OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.into()),
                    )?;
                    state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.into()))?;
                    state.set_tag(ParserState::TagName as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagCloseBeforeNamePlusWhitespace => match last_symbol {
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(
                        state,
                        OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.into()),
                    )?;
                    state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.into()))?;
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
                    push_to_state(state, OutputSymbol::TagPadding)?;
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagName(last_symbol.into()))?;
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
                    last_symbol.to_string(),
                    "Invalid expression block start".to_string(),
                )
                .into_err(Position::NONE)),
                _ if last_symbol.trim().is_empty() => {
                    push_to_state(state, OutputSymbol::TagPadding)?;
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                "/" => {
                    push_to_state(state, OutputSymbol::TagSelfClose)?;
                    state.set_tag(ParserState::TagSelfClose as i32);

                    Ok(Some(">".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.into()))?;
                    state.set_tag(ParserState::TagAttributeName as i32);

                    Ok(Some("$raw$".into()))
                }
            },
            ParserState::TagAttributeName => match last_symbol {
                "=" => {
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
                    push_to_state(state, OutputSymbol::TagPadding)?;
                    state.set_tag(ParserState::TagContent as i32);

                    Ok(Some("$raw$".into()))
                }
                _ => {
                    push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.into()))?;
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
                    push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.into()))?;
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
                        OutputSymbol::TagAttributeValueString(last_symbol.into()),
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
                    last_symbol.to_string(),
                    "Invalid self-closing tag end".to_string(),
                )
                .into_err(Position::NONE)),
            },
        },
        Err(_) => Err(LexError::ImproperSymbol(
            last_symbol.to_string(),
            "Invalid parser state".to_string(),
        )
        .into_err(Position::NONE)),
    }
}
