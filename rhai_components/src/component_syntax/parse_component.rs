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

fn push_to_state(state: &mut Dynamic, value: OutputSymbol) {
    if let Ok(mut array) = state.as_array_mut() {
        array.push(Dynamic::from(value));
    }
}

pub fn parse_component(
    symbols: &[ImmutableString],
    state: &mut Dynamic,
) -> Result<Option<ImmutableString>, ParseError> {
    let last_symbol = symbols
        .last()
        .ok_or_else(|| LexError::Runtime("No symbols found".to_string()).into_err(Position::NONE))?
        .as_str();

    let current_state = ParserState::try_from(state.tag()).map_err(|()| {
        LexError::ImproperSymbol(last_symbol.to_string(), "Invalid parser state".to_string())
            .into_err(Position::NONE)
    })?;

    if !matches!(
        current_state,
        ParserState::Start | ParserState::OpeningBracket
    ) {
        state.as_array_ref().map_err(|err| {
            LexError::Runtime(format!("Invalid state array {err} at token: {last_symbol}"))
                .into_err(Position::NONE)
        })?;
    }

    match current_state {
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
                push_to_state(state, OutputSymbol::TagLeftAnglePlusWhitespace);
                state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

                Ok(Some("$raw$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::Text(last_symbol.to_string()));
                state.set_tag(ParserState::Body as i32);

                Ok(Some("$raw$".into()))
            }
        },
        ParserState::BodyExpression => match last_symbol {
            "$inner$" => {
                push_to_state(state, OutputSymbol::BodyExpression);

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
                push_to_state(state, OutputSymbol::TagLeftAnglePlusWhitespace);
                state.set_tag(ParserState::TagLeftAnglePlusWhitespace as i32);

                Ok(Some("$raw$".into()))
            }
            "/" => {
                push_to_state(
                    state,
                    OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.to_string()),
                );
                state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                Ok(Some("$raw$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()));
                state.set_tag(ParserState::TagName as i32);

                Ok(Some("$raw$".into()))
            }
        },
        ParserState::TagCloseBeforeNamePlusWhitespace => match last_symbol {
            _ if last_symbol.trim().is_empty() => {
                push_to_state(
                    state,
                    OutputSymbol::TagCloseBeforeNamePlusWhitespace(last_symbol.to_string()),
                );
                state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

                Ok(Some("$raw$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()));
                state.set_tag(ParserState::TagName as i32);

                Ok(Some("$raw$".into()))
            }
        },
        ParserState::TagName => match last_symbol {
            ">" => {
                push_to_state(state, OutputSymbol::TagRightAngle);
                state.set_tag(ParserState::Body as i32);

                Ok(Some("$raw$".into()))
            }
            _ if last_symbol.trim().is_empty() => {
                push_to_state(state, OutputSymbol::TagPadding);
                state.set_tag(ParserState::TagContent as i32);

                Ok(Some("$raw$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagName(last_symbol.to_string()));
                state.set_tag(ParserState::TagName as i32);

                Ok(Some("$raw$".into()))
            }
        },
        ParserState::TagContent => match last_symbol {
            ">" => {
                push_to_state(state, OutputSymbol::TagRightAngle);
                state.set_tag(ParserState::Body as i32);

                Ok(Some("$raw$".into()))
            }
            "{" => Err(LexError::ImproperSymbol(
                last_symbol.to_string(),
                "Invalid expression block start".to_string(),
            )
            .into_err(Position::NONE)),
            _ if last_symbol.trim().is_empty() => {
                push_to_state(state, OutputSymbol::TagPadding);
                state.set_tag(ParserState::TagContent as i32);

                Ok(Some("$raw$".into()))
            }
            "/" => {
                push_to_state(state, OutputSymbol::TagSelfClose);
                state.set_tag(ParserState::TagSelfClose as i32);

                Ok(Some(">".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.to_string()));
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
                push_to_state(state, OutputSymbol::TagRightAngle);
                state.set_tag(ParserState::Body as i32);

                Ok(Some("$raw$".into()))
            }
            "/" => {
                push_to_state(state, OutputSymbol::TagSelfClose);
                state.set_tag(ParserState::TagSelfClose as i32);

                Ok(Some(">".into()))
            }
            _ if last_symbol.trim().is_empty() => {
                push_to_state(state, OutputSymbol::TagPadding);
                state.set_tag(ParserState::TagContent as i32);

                Ok(Some("$raw$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.to_string()));
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
                push_to_state(state, OutputSymbol::TagAttributeValueExpression);
                state.set_tag(ParserState::TagAttributeValue as i32);

                Ok(Some("$inner$".into()))
            }
            _ => {
                push_to_state(state, OutputSymbol::TagAttributeName(last_symbol.to_string()));
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
                );
                state.set_tag(ParserState::TagAttributeValueString as i32);

                Ok(Some("$raw$".into()))
            }
        },
        ParserState::TagSelfClose => match last_symbol {
            ">" => {
                push_to_state(state, OutputSymbol::TagRightAngle);
                state.set_tag(ParserState::Body as i32);

                Ok(Some("$raw$".into()))
            }
            _ => Err(LexError::ImproperSymbol(
                last_symbol.to_string(),
                "Invalid self-closing tag end".to_string(),
            )
            .into_err(Position::NONE)),
        },
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::ImmutableString;

    use super::ParserState;
    use super::parse_component;

    fn symbols(values: &[&str]) -> Vec<ImmutableString> {
        values.iter().map(|value| (*value).into()).collect()
    }

    fn fresh_state() -> Dynamic {
        let mut state = Dynamic::from_array(Vec::new());

        state.set_tag(ParserState::Body as i32);

        state
    }

    #[test]
    fn errs_when_symbols_slice_is_empty() -> Result<()> {
        let mut state: Dynamic = Dynamic::UNIT;

        assert!(parse_component(&[], &mut state)
            .is_err_and(|error| error.to_string().contains("No symbols found")));

        Ok(())
    }

    #[test]
    fn errs_on_invalid_parser_state_tag() -> Result<()> {
        let inputs = symbols(&["x"]);
        let mut state = fresh_state();

        state.set_tag(99);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Invalid parser state")));

        Ok(())
    }

    #[test]
    fn errs_on_invalid_expression_block_end_in_body_expression() -> Result<()> {
        let inputs = symbols(&["x"]);
        let mut state = fresh_state();

        state.set_tag(ParserState::BodyExpression as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Invalid expression block end")));

        Ok(())
    }

    #[test]
    fn errs_on_invalid_expression_block_start_in_tag_content() -> Result<()> {
        let inputs = symbols(&["{"]);
        let mut state = fresh_state();

        state.set_tag(ParserState::TagContent as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Invalid expression block start")));

        Ok(())
    }

    #[test]
    fn errs_on_invalid_self_close_end() -> Result<()> {
        let inputs = symbols(&["x"]);
        let mut state = fresh_state();

        state.set_tag(ParserState::TagSelfClose as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Invalid self-closing tag end")));

        Ok(())
    }

    #[test]
    fn errs_when_push_to_state_finds_non_array_state() -> Result<()> {
        let inputs = symbols(&["<"]);
        let mut state = Dynamic::from(42_i64);

        state.set_tag(ParserState::Body as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Invalid state array")));

        Ok(())
    }

    #[test]
    fn keeps_collecting_whitespace_after_close_slash() -> Result<()> {
        let inputs = symbols(&[" "]);
        let mut state = fresh_state();

        state.set_tag(ParserState::TagCloseBeforeNamePlusWhitespace as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_ok_and(|next| next.as_deref() == Some("$raw$")));
        assert_eq!(state.tag(), ParserState::TagCloseBeforeNamePlusWhitespace as i32);

        Ok(())
    }

    #[test]
    fn switches_to_self_close_when_slash_follows_attribute_name() -> Result<()> {
        let inputs = symbols(&["/"]);
        let mut state = fresh_state();

        state.set_tag(ParserState::TagAttributeName as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_ok_and(|next| next.as_deref() == Some(">")));
        assert_eq!(state.tag(), ParserState::TagSelfClose as i32);

        Ok(())
    }

    #[test]
    fn treats_unquoted_attribute_value_as_new_attribute_name() -> Result<()> {
        let inputs = symbols(&["y"]);
        let mut state = fresh_state();

        state.set_tag(ParserState::TagAttributeValue as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_ok_and(|next| next.as_deref() == Some("$raw$")));
        assert_eq!(state.tag(), ParserState::TagContent as i32);

        Ok(())
    }

    #[test]
    fn push_to_state_silently_skips_non_array_state() -> Result<()> {
        let mut state = Dynamic::from(42_i64);

        super::push_to_state(&mut state, super::OutputSymbol::Text("x".to_string()));

        assert!(state.as_array_ref().is_err());

        Ok(())
    }

    #[test]
    fn propagates_combine_output_symbols_error_when_body_closes() -> Result<()> {
        let inputs = symbols(&["}"]);
        let mut state = Dynamic::from_array(vec![Dynamic::from(super::OutputSymbol::TagName(
            "loose-name".to_string(),
        ))]);

        state.set_tag(ParserState::Body as i32);

        assert!(parse_component(&inputs, &mut state)
            .is_err_and(|error| error.to_string().contains("Unexpected tag name")));

        Ok(())
    }
}
