use std::collections::VecDeque;

use rhai::Dynamic;
use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::attribute::Attribute;
use super::attribute_value::AttributeValue;
use super::expression_reference::ExpressionReference;
use super::output_combined_symbol::OutputCombinedSymbol;
use super::output_semantic_symbol::OutputSemanticSymbol;
use super::output_symbol::OutputSymbol;
use super::tag::Tag;
use super::tag_kind::TagKind;
use super::tag_name::TagName;

fn merge_adjacent_symbols(state: &Dynamic) -> Result<Vec<OutputCombinedSymbol>, ParseError> {
    let mut expression_index = 0;
    let mut combined_symbols: Vec<OutputCombinedSymbol> = vec![];

    let state_array = match state.as_array_ref() {
        Ok(array) => array,
        Err(err) => {
            return Err(
                LexError::Runtime(format!("Invalid state array {err}")).into_err(Position::NONE)
            );
        }
    };

    for node in state_array.iter() {
        match node.clone().try_cast::<OutputSymbol>().ok_or_else(|| {
            LexError::Runtime("Unable to cast state to output symbols".to_owned())
                .into_err(Position::NONE)
        })? {
            OutputSymbol::BodyExpression => {
                combined_symbols.push(OutputCombinedSymbol::BodyExpression(ExpressionReference {
                    expression_index,
                }));
                expression_index += 1;
            }
            OutputSymbol::TagAttributeValueExpression => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagAttributeName(_)) => {
                    combined_symbols.push(OutputCombinedSymbol::TagAttributeValue(
                        AttributeValue::Expression(ExpressionReference { expression_index }),
                    ));
                    expression_index += 1;
                }
                _ => {
                    return Err(LexError::Runtime(
                        "Attribute value expression without name".to_owned(),
                    )
                    .into_err(Position::NONE));
                }
            },
            OutputSymbol::TagLeftAnglePlusWhitespace => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagLeftAngle) => {}
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagLeftAngle);
                }
            },
            OutputSymbol::TagCloseBeforeNamePlusWhitespace => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagCloseBeforeName) => {}
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagCloseBeforeName);
                }
            },
            OutputSymbol::TagPadding => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagPadding) => {}
                _ => {
                    combined_symbols.push(OutputCombinedSymbol::TagPadding);
                }
            },
            OutputSymbol::TagAttributeValueString(text) => match combined_symbols.last_mut() {
                Some(OutputCombinedSymbol::TagAttributeName(_)) => {
                    combined_symbols.push(OutputCombinedSymbol::TagAttributeValue(
                        AttributeValue::Text(text),
                    ));
                }
                Some(OutputCombinedSymbol::TagAttributeValue(AttributeValue::Text(value))) => {
                    value.push_str(&text);
                }
                _ => {
                    return Err(LexError::Runtime(
                        "Attribute value expression without name".to_owned(),
                    )
                    .into_err(Position::NONE));
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

    Ok(combined_symbols)
}

fn assemble_semantic_symbols(
    combined_symbols: Vec<OutputCombinedSymbol>,
) -> Result<VecDeque<OutputSemanticSymbol>, ParseError> {
    let mut semantic_symbols: VecDeque<OutputSemanticSymbol> = VecDeque::new();

    for output_combined_symbol in combined_symbols {
        match output_combined_symbol {
            OutputCombinedSymbol::BodyExpression(expression_reference) => {
                semantic_symbols
                    .push_back(OutputSemanticSymbol::BodyExpression(expression_reference));
            }
            OutputCombinedSymbol::Text(text) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Text(existing_text)) => {
                    existing_text.push_str(&text);
                }
                _ => {
                    semantic_symbols.push_back(OutputSemanticSymbol::Text(text));
                }
            },
            OutputCombinedSymbol::TagLeftAngle => match semantic_symbols.back_mut() {
                Some(
                    OutputSemanticSymbol::BodyExpression(_)
                    | OutputSemanticSymbol::Tag(_)
                    | OutputSemanticSymbol::Text(_),
                ) => {
                    semantic_symbols.push_back(OutputSemanticSymbol::Tag(Tag {
                        attributes: vec![],
                        kind: TagKind::Opening,
                        tag_name: TagName {
                            name: String::new(),
                        },
                    }));
                }
                last_symbol => {
                    return Err(LexError::UnexpectedInput(format!(
                        "Unexpected tag opening after {last_symbol:?}"
                    ))
                    .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagCloseBeforeName => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { kind, .. })) => {
                    *kind = TagKind::Closing;
                }
                _ => {
                    return Err(
                        LexError::UnexpectedInput("Unexpected tag closing".to_owned())
                            .into_err(Position::NONE),
                    );
                }
            },
            OutputCombinedSymbol::TagName(name) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag {
                    tag_name: existing_name,
                    ..
                })) => {
                    existing_name.name = name;
                }
                _ => {
                    return Err(LexError::UnexpectedInput("Unexpected tag name".to_owned())
                        .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagAttributeName(name) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                    attributes.push(Attribute {
                        name,
                        value: AttributeValue::Empty,
                    });
                }
                _ => {
                    return Err(LexError::UnexpectedInput(
                        "Unexpected tag attribute name".to_owned(),
                    )
                    .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagAttributeValue(attribute_value) => {
                match semantic_symbols.back_mut() {
                    Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                        if let Some(last_attribute) = attributes.last_mut() {
                            last_attribute.value = attribute_value;
                        } else {
                            return Err(LexError::UnexpectedInput(
                                "Attribute value without name".to_owned(),
                            )
                            .into_err(Position::NONE));
                        }
                    }
                    _ => {
                        return Err(LexError::UnexpectedInput(
                            "Unexpected tag attribute value".to_owned(),
                        )
                        .into_err(Position::NONE));
                    }
                }
            }
            OutputCombinedSymbol::TagSelfClose => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { kind, .. })) => {
                    *kind = TagKind::SelfClosing;
                }
                _ => {
                    return Err(LexError::UnexpectedInput(
                        "Unexpected self-closing tag".to_owned(),
                    )
                    .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagPadding | OutputCombinedSymbol::TagRightAngle => {}
        }
    }

    Ok(semantic_symbols)
}

pub fn combine_output_symbols(
    state: &Dynamic,
) -> Result<VecDeque<OutputSemanticSymbol>, ParseError> {
    assemble_semantic_symbols(merge_adjacent_symbols(state)?)
}

#[cfg(test)]
mod tests {
    use rhai::Dynamic;
    use rhai::LexError;
    use rhai::ParseErrorType;

    use super::AttributeValue;
    use super::OutputCombinedSymbol;
    use super::OutputSemanticSymbol;
    use super::OutputSymbol;
    use super::assemble_semantic_symbols;
    use super::combine_output_symbols;
    use super::merge_adjacent_symbols;

    fn state_of(symbols: Vec<OutputSymbol>) -> Dynamic {
        Dynamic::from_array(symbols.into_iter().map(Dynamic::from).collect())
    }

    fn combining_error(state: &Dynamic) -> Option<ParseErrorType> {
        combine_output_symbols(state)
            .err()
            .map(|parse_error| parse_error.err_type().clone())
    }

    fn assembling_error(combined_symbols: Vec<OutputCombinedSymbol>) -> Option<ParseErrorType> {
        assemble_semantic_symbols(combined_symbols)
            .err()
            .map(|parse_error| parse_error.err_type().clone())
    }

    fn runtime_error(message: &str) -> ParseErrorType {
        ParseErrorType::BadInput(LexError::Runtime(message.to_owned()))
    }

    fn unexpected_input(message: &str) -> ParseErrorType {
        ParseErrorType::BadInput(LexError::UnexpectedInput(message.to_owned()))
    }

    #[test]
    fn rejects_state_that_is_not_an_array() {
        assert_eq!(
            combining_error(&Dynamic::from(42_i64)),
            Some(runtime_error("Invalid state array i64"))
        );
    }

    #[test]
    fn rejects_state_array_with_foreign_values() {
        assert_eq!(
            combining_error(&Dynamic::from_array(vec![Dynamic::from(42_i64)])),
            Some(runtime_error("Unable to cast state to output symbols"))
        );
    }

    #[test]
    fn rejects_attribute_value_expression_without_attribute_name() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagAttributeValueExpression])),
            Some(runtime_error("Attribute value expression without name"))
        );
    }

    #[test]
    fn rejects_attribute_value_string_without_attribute_name() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagAttributeValueString(
                "value".to_owned()
            )])),
            Some(runtime_error("Attribute value expression without name"))
        );
    }

    #[test]
    fn rejects_tag_opening_at_start() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagLeftAnglePlusWhitespace])),
            Some(unexpected_input("Unexpected tag opening after None"))
        );
    }

    #[test]
    fn rejects_tag_closing_without_tag() {
        assert_eq!(
            combining_error(&state_of(vec![
                OutputSymbol::TagCloseBeforeNamePlusWhitespace
            ])),
            Some(unexpected_input("Unexpected tag closing"))
        );
    }

    #[test]
    fn rejects_tag_name_without_tag() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagName("div".to_owned())])),
            Some(unexpected_input("Unexpected tag name"))
        );
    }

    #[test]
    fn rejects_attribute_name_without_tag() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagAttributeName(
                "class".to_owned()
            )])),
            Some(unexpected_input("Unexpected tag attribute name"))
        );
    }

    #[test]
    fn rejects_self_close_without_tag() {
        assert_eq!(
            combining_error(&state_of(vec![OutputSymbol::TagSelfClose])),
            Some(unexpected_input("Unexpected self-closing tag"))
        );
    }

    #[test]
    fn rejects_attribute_value_before_attribute_name() {
        assert_eq!(
            assembling_error(vec![
                OutputCombinedSymbol::Text("text".to_owned()),
                OutputCombinedSymbol::TagLeftAngle,
                OutputCombinedSymbol::TagAttributeValue(AttributeValue::Text("value".to_owned())),
            ]),
            Some(unexpected_input("Attribute value without name"))
        );
    }

    #[test]
    fn rejects_attribute_value_without_tag() {
        assert_eq!(
            assembling_error(vec![OutputCombinedSymbol::TagAttributeValue(
                AttributeValue::Text("value".to_owned())
            )]),
            Some(unexpected_input("Unexpected tag attribute value"))
        );
    }

    #[test]
    fn merges_consecutive_symbols_of_the_same_kind() {
        assert!(
            merge_adjacent_symbols(&state_of(vec![
                OutputSymbol::Text("a".to_owned()),
                OutputSymbol::Text("b".to_owned()),
                OutputSymbol::TagLeftAnglePlusWhitespace,
                OutputSymbol::TagLeftAnglePlusWhitespace,
                OutputSymbol::TagCloseBeforeNamePlusWhitespace,
                OutputSymbol::TagCloseBeforeNamePlusWhitespace,
                OutputSymbol::TagName("d".to_owned()),
                OutputSymbol::TagName("iv".to_owned()),
                OutputSymbol::TagPadding,
                OutputSymbol::TagPadding,
                OutputSymbol::TagAttributeName("c".to_owned()),
                OutputSymbol::TagAttributeName("lass".to_owned()),
            ]))
            .is_ok_and(|combined_symbols| {
                combined_symbols
                    == vec![
                        OutputCombinedSymbol::Text("ab".to_owned()),
                        OutputCombinedSymbol::TagLeftAngle,
                        OutputCombinedSymbol::TagCloseBeforeName,
                        OutputCombinedSymbol::TagName("div".to_owned()),
                        OutputCombinedSymbol::TagPadding,
                        OutputCombinedSymbol::TagAttributeName("class".to_owned()),
                    ]
            })
        );
    }

    #[test]
    fn merges_consecutive_text_into_one_semantic_symbol() {
        assert!(
            assemble_semantic_symbols(vec![
                OutputCombinedSymbol::Text("a".to_owned()),
                OutputCombinedSymbol::Text("b".to_owned()),
            ])
            .is_ok_and(|semantic_symbols| {
                semantic_symbols.len() == 1
                    && matches!(
                        semantic_symbols.front(),
                        Some(OutputSemanticSymbol::Text(text)) if text == "ab"
                    )
            })
        );
    }
}
