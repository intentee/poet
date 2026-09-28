use rhai::Dynamic;
use rhai::ImmutableString;
use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::combine_output_symbols::combine_output_symbols;
use super::combine_tag_stack::combine_tag_stack;
use super::output_symbol::OutputSymbol;
use super::parser_state::ParserState;
use super::parser_step::ParserStep;

const RAW_SYMBOL: &str = "$raw$";
const INNER_SYMBOL: &str = "$inner$";

const fn emit(
    output_symbol: OutputSymbol,
    next_state: ParserState,
    next_symbol: &'static str,
) -> ParserStep {
    ParserStep::Emit {
        output_symbol,
        next_state,
        next_symbol,
    }
}

fn improper_symbol(last_symbol: &str, message: &str) -> ParseError {
    LexError::ImproperSymbol(last_symbol.to_owned(), message.to_owned()).into_err(Position::NONE)
}

fn is_whitespace(last_symbol: &str) -> bool {
    last_symbol.trim().is_empty()
}

fn next_parser_step(
    current_state: ParserState,
    last_symbol: &str,
) -> Result<ParserStep, ParseError> {
    Ok(match current_state {
        ParserState::Start => ParserStep::Begin,
        ParserState::OpeningBracket => ParserStep::Advance {
            next_state: ParserState::Body,
            next_symbol: RAW_SYMBOL,
        },
        ParserState::Body => match last_symbol {
            "{" => ParserStep::Advance {
                next_state: ParserState::BodyExpression,
                next_symbol: INNER_SYMBOL,
            },
            "}" => ParserStep::Finish,
            "<" => emit(
                OutputSymbol::TagLeftAnglePlusWhitespace,
                ParserState::TagLeftAnglePlusWhitespace,
                RAW_SYMBOL,
            ),
            _ => emit(
                OutputSymbol::Text(last_symbol.to_owned()),
                ParserState::Body,
                RAW_SYMBOL,
            ),
        },
        ParserState::BodyExpression => match last_symbol {
            INNER_SYMBOL => emit(OutputSymbol::BodyExpression, ParserState::Body, RAW_SYMBOL),
            _ => return Err(improper_symbol(last_symbol, "Invalid expression block end")),
        },
        ParserState::TagLeftAnglePlusWhitespace => match last_symbol {
            _ if is_whitespace(last_symbol) => emit(
                OutputSymbol::TagLeftAnglePlusWhitespace,
                ParserState::TagLeftAnglePlusWhitespace,
                RAW_SYMBOL,
            ),
            "/" => emit(
                OutputSymbol::TagCloseBeforeNamePlusWhitespace,
                ParserState::TagCloseBeforeNamePlusWhitespace,
                RAW_SYMBOL,
            ),
            _ => emit(
                OutputSymbol::TagName(last_symbol.to_owned()),
                ParserState::TagName,
                RAW_SYMBOL,
            ),
        },
        ParserState::TagCloseBeforeNamePlusWhitespace => {
            if is_whitespace(last_symbol) {
                emit(
                    OutputSymbol::TagCloseBeforeNamePlusWhitespace,
                    ParserState::TagCloseBeforeNamePlusWhitespace,
                    RAW_SYMBOL,
                )
            } else {
                emit(
                    OutputSymbol::TagName(last_symbol.to_owned()),
                    ParserState::TagName,
                    RAW_SYMBOL,
                )
            }
        }
        ParserState::TagName => match last_symbol {
            ">" => emit(OutputSymbol::TagRightAngle, ParserState::Body, RAW_SYMBOL),
            _ if is_whitespace(last_symbol) => emit(
                OutputSymbol::TagPadding,
                ParserState::TagContent,
                RAW_SYMBOL,
            ),
            _ => emit(
                OutputSymbol::TagName(last_symbol.to_owned()),
                ParserState::TagName,
                RAW_SYMBOL,
            ),
        },
        ParserState::TagContent => match last_symbol {
            ">" => emit(OutputSymbol::TagRightAngle, ParserState::Body, RAW_SYMBOL),
            "{" => {
                return Err(improper_symbol(
                    last_symbol,
                    "Invalid expression block start",
                ));
            }
            _ if is_whitespace(last_symbol) => emit(
                OutputSymbol::TagPadding,
                ParserState::TagContent,
                RAW_SYMBOL,
            ),
            "/" => emit(OutputSymbol::TagSelfClose, ParserState::TagSelfClose, ">"),
            _ => emit(
                OutputSymbol::TagAttributeName(last_symbol.to_owned()),
                ParserState::TagAttributeName,
                RAW_SYMBOL,
            ),
        },
        ParserState::TagAttributeName => match last_symbol {
            "=" => ParserStep::Advance {
                next_state: ParserState::TagAttributeValue,
                next_symbol: RAW_SYMBOL,
            },
            ">" => emit(OutputSymbol::TagRightAngle, ParserState::Body, RAW_SYMBOL),
            "/" => emit(OutputSymbol::TagSelfClose, ParserState::TagSelfClose, ">"),
            _ if is_whitespace(last_symbol) => emit(
                OutputSymbol::TagPadding,
                ParserState::TagContent,
                RAW_SYMBOL,
            ),
            _ => emit(
                OutputSymbol::TagAttributeName(last_symbol.to_owned()),
                ParserState::TagAttributeName,
                RAW_SYMBOL,
            ),
        },
        ParserState::TagAttributeValue => match last_symbol {
            INNER_SYMBOL => ParserStep::Advance {
                next_state: ParserState::TagContent,
                next_symbol: RAW_SYMBOL,
            },
            "\"" => ParserStep::Advance {
                next_state: ParserState::TagAttributeValueString,
                next_symbol: RAW_SYMBOL,
            },
            "{" => emit(
                OutputSymbol::TagAttributeValueExpression,
                ParserState::TagAttributeValue,
                INNER_SYMBOL,
            ),
            _ => emit(
                OutputSymbol::TagAttributeName(last_symbol.to_owned()),
                ParserState::TagContent,
                RAW_SYMBOL,
            ),
        },
        ParserState::TagAttributeValueString => {
            if last_symbol == "\"" {
                ParserStep::Advance {
                    next_state: ParserState::TagContent,
                    next_symbol: RAW_SYMBOL,
                }
            } else {
                emit(
                    OutputSymbol::TagAttributeValueString(last_symbol.to_owned()),
                    ParserState::TagAttributeValueString,
                    RAW_SYMBOL,
                )
            }
        }
        ParserState::TagSelfClose => match last_symbol {
            ">" => emit(OutputSymbol::TagRightAngle, ParserState::Body, RAW_SYMBOL),
            _ => return Err(improper_symbol(last_symbol, "Invalid self-closing tag end")),
        },
    })
}

fn apply_parser_step(
    parser_step: ParserStep,
    state: &mut Dynamic,
) -> Result<Option<ImmutableString>, ParseError> {
    match parser_step {
        ParserStep::Advance {
            next_state,
            next_symbol,
        } => {
            state.set_tag(next_state as i32);

            Ok(Some(next_symbol.into()))
        }
        ParserStep::Begin => {
            *state = Dynamic::from_array(vec![]);
            state.set_tag(ParserState::OpeningBracket as i32);

            Ok(Some("{".into()))
        }
        ParserStep::Emit {
            output_symbol,
            next_state,
            next_symbol,
        } => {
            state
                .as_array_mut()
                .map_err(|state_type_name| {
                    LexError::Runtime(format!("Invalid state array {state_type_name}"))
                        .into_err(Position::NONE)
                })?
                .push(Dynamic::from(output_symbol));
            state.set_tag(next_state as i32);

            Ok(Some(next_symbol.into()))
        }
        ParserStep::Finish => {
            let mut semantic_symbols = combine_output_symbols(state)?;

            *state = Dynamic::from(combine_tag_stack(&mut semantic_symbols)?);

            Ok(None)
        }
    }
}

pub fn parse_component(
    symbols: &[ImmutableString],
    state: &mut Dynamic,
) -> Result<Option<ImmutableString>, ParseError> {
    let last_symbol = symbols
        .last()
        .ok_or_else(|| LexError::Runtime("No symbols found".to_owned()).into_err(Position::NONE))?
        .as_str();
    let current_state = ParserState::try_from(state.tag())
        .map_err(|lex_error| lex_error.into_err(Position::NONE))?;

    apply_parser_step(next_parser_step(current_state, last_symbol)?, state)
}

#[cfg(test)]
mod tests {
    use rhai::Dynamic;
    use rhai::ImmutableString;
    use rhai::LexError;
    use rhai::ParseErrorType;

    use super::OutputSymbol;
    use super::ParserState;
    use super::parse_component;

    fn symbols(values: &[&str]) -> Vec<ImmutableString> {
        values.iter().map(|value| (*value).into()).collect()
    }

    fn state_in(parser_state: ParserState) -> Dynamic {
        let mut state = Dynamic::from_array(Vec::new());

        state.set_tag(parser_state as i32);

        state
    }

    fn parse_error(symbol_values: &[&str], state: &mut Dynamic) -> Option<ParseErrorType> {
        parse_component(&symbols(symbol_values), state)
            .err()
            .map(|parse_error| parse_error.err_type().clone())
    }

    fn improper_symbol(symbol: &str, message: &str) -> ParseErrorType {
        ParseErrorType::BadInput(LexError::ImproperSymbol(
            symbol.to_owned(),
            message.to_owned(),
        ))
    }

    #[test]
    fn rejects_empty_symbols() {
        let mut state = Dynamic::UNIT;

        assert_eq!(
            parse_error(&[], &mut state),
            Some(ParseErrorType::BadInput(LexError::Runtime(
                "No symbols found".to_owned()
            )))
        );
    }

    #[test]
    fn rejects_unknown_parser_state() {
        let mut state = state_in(ParserState::Body);

        state.set_tag(99);

        assert_eq!(
            parse_error(&["x"], &mut state),
            Some(ParseErrorType::BadInput(LexError::Runtime(
                "Invalid parser state 99".to_owned()
            )))
        );
    }

    #[test]
    fn rejects_unterminated_body_expression() {
        assert_eq!(
            parse_error(&["x"], &mut state_in(ParserState::BodyExpression)),
            Some(improper_symbol("x", "Invalid expression block end"))
        );
    }

    #[test]
    fn rejects_expression_block_inside_tag_content() {
        assert_eq!(
            parse_error(&["{"], &mut state_in(ParserState::TagContent)),
            Some(improper_symbol("{", "Invalid expression block start"))
        );
    }

    #[test]
    fn rejects_self_close_without_right_angle() {
        assert_eq!(
            parse_error(&["x"], &mut state_in(ParserState::TagSelfClose)),
            Some(improper_symbol("x", "Invalid self-closing tag end"))
        );
    }

    #[test]
    fn rejects_state_that_is_not_an_array() {
        let mut state = Dynamic::from(42_i64);

        state.set_tag(ParserState::Body as i32);

        assert_eq!(
            parse_error(&["<"], &mut state),
            Some(ParseErrorType::BadInput(LexError::Runtime(
                "Invalid state array i64".to_owned()
            )))
        );
    }

    #[test]
    fn keeps_collecting_whitespace_after_close_slash() {
        let mut state = state_in(ParserState::TagCloseBeforeNamePlusWhitespace);

        assert!(
            parse_component(&symbols(&[" "]), &mut state)
                .is_ok_and(|next_symbol| next_symbol.as_deref() == Some("$raw$"))
        );
        assert_eq!(
            state.tag(),
            ParserState::TagCloseBeforeNamePlusWhitespace as i32
        );
    }

    #[test]
    fn switches_to_self_close_when_slash_follows_attribute_name() {
        let mut state = state_in(ParserState::TagAttributeName);

        assert!(
            parse_component(&symbols(&["/"]), &mut state)
                .is_ok_and(|next_symbol| next_symbol.as_deref() == Some(">"))
        );
        assert_eq!(state.tag(), ParserState::TagSelfClose as i32);
    }

    #[test]
    fn treats_unquoted_attribute_value_as_new_attribute_name() {
        let mut state = state_in(ParserState::TagAttributeValue);

        assert!(
            parse_component(&symbols(&["y"]), &mut state)
                .is_ok_and(|next_symbol| next_symbol.as_deref() == Some("$raw$"))
        );
        assert_eq!(state.tag(), ParserState::TagContent as i32);
    }

    #[test]
    fn propagates_symbol_combination_error_when_body_closes() {
        let mut state = Dynamic::from_array(vec![Dynamic::from(OutputSymbol::TagName(
            "loose-name".to_owned(),
        ))]);

        state.set_tag(ParserState::Body as i32);

        assert_eq!(
            parse_error(&["}"], &mut state),
            Some(ParseErrorType::BadInput(LexError::UnexpectedInput(
                "Unexpected tag name".to_owned()
            )))
        );
    }
}
