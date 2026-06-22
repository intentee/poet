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
use crate::component_syntax::tag_name::TagName;

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
            LexError::Runtime("Unable to cast state to output symbols".to_string())
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
                        "Attribute value expression without name".to_string(),
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
            OutputSymbol::TagCloseBeforeNamePlusWhitespace(_text) => {
                match combined_symbols.last_mut() {
                    Some(OutputCombinedSymbol::TagCloseBeforeName) => {}
                    _ => {
                        combined_symbols.push(OutputCombinedSymbol::TagCloseBeforeName);
                    }
                }
            }
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
                        "Attribute value expression without name".to_string(),
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
                    semantic_symbols.push_back(OutputSemanticSymbol::Text(text.to_string()));
                }
            },
            OutputCombinedSymbol::TagLeftAngle => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::BodyExpression(_))
                | Some(OutputSemanticSymbol::Tag(_))
                | Some(OutputSemanticSymbol::Text(_)) => {
                    semantic_symbols.push_back(OutputSemanticSymbol::Tag(Tag {
                        attributes: vec![],
                        is_closing: false,
                        is_self_closing: false,
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
                Some(OutputSemanticSymbol::Tag(Tag { is_closing, .. })) => {
                    *is_closing = true;
                }
                _ => {
                    return Err(
                        LexError::UnexpectedInput("Unexpected tag closing".to_string())
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
                    return Err(LexError::UnexpectedInput("Unexpected tag name".to_string())
                        .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagAttributeName(name) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                    attributes.push(Attribute { name, value: None });
                }
                _ => {
                    return Err(LexError::UnexpectedInput(
                        "Unexpected tag attribute name".to_string(),
                    )
                    .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagAttributeValue(attribute_value) => {
                match semantic_symbols.back_mut() {
                    Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                        if let Some(last_attribute) = attributes.last_mut() {
                            last_attribute.value = Some(attribute_value);
                        } else {
                            return Err(LexError::UnexpectedInput(
                                "Attribute value without name".to_string(),
                            )
                            .into_err(Position::NONE));
                        }
                    }
                    _ => {
                        return Err(LexError::UnexpectedInput(
                            "Unexpected tag attribute value".to_string(),
                        )
                        .into_err(Position::NONE));
                    }
                }
            }
            OutputCombinedSymbol::TagPadding => {}
            OutputCombinedSymbol::TagSelfClose => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag {
                    is_self_closing, ..
                })) => {
                    *is_self_closing = true;
                }
                _ => {
                    return Err(LexError::UnexpectedInput(
                        "Unexpected self-closing tag".to_string(),
                    )
                    .into_err(Position::NONE));
                }
            },
            OutputCombinedSymbol::TagRightAngle => {}
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
    use std::mem::discriminant;

    use anyhow::Result;
    use rhai::Dynamic;

    use super::AttributeValue;
    use super::OutputCombinedSymbol;
    use super::OutputSemanticSymbol;
    use super::OutputSymbol;
    use super::assemble_semantic_symbols;
    use super::combine_output_symbols;
    use super::merge_adjacent_symbols;

    fn make_state(symbols: Vec<OutputSymbol>) -> Dynamic {
        Dynamic::from_array(symbols.into_iter().map(Dynamic::from).collect())
    }

    #[test]
    fn errs_when_state_is_not_an_array() -> Result<()> {
        let state = Dynamic::from(42_i64);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Invalid state array"))
        );

        Ok(())
    }

    #[test]
    fn errs_when_state_array_contains_non_output_symbol() -> Result<()> {
        let state = Dynamic::from_array(vec![Dynamic::from(42_i64)]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unable to cast"))
        );

        Ok(())
    }

    #[test]
    fn errs_when_attribute_value_expression_has_no_prior_attribute_name() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagAttributeValueExpression]);

        assert!(combine_output_symbols(&state).is_err_and(|error| {
            error
                .to_string()
                .contains("Attribute value expression without name")
        }));

        Ok(())
    }

    #[test]
    fn errs_when_attribute_value_string_has_no_prior_attribute_name() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagAttributeValueString("v".to_string())]);

        assert!(combine_output_symbols(&state).is_err_and(|error| {
            error
                .to_string()
                .contains("Attribute value expression without name")
        }));

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_tag_opening_after_unsupported_predecessor() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagLeftAnglePlusWhitespace]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unexpected tag opening"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_tag_closing_with_no_open_tag() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagCloseBeforeNamePlusWhitespace(
            "".to_string(),
        )]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unexpected tag closing"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_tag_name_with_no_open_tag() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagName("d".to_string())]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unexpected tag name"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_attribute_name_with_no_open_tag() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagAttributeName("c".to_string())]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unexpected tag attribute name"))
        );

        Ok(())
    }

    #[test]
    fn errs_when_attribute_value_emitted_before_attribute_name_in_assemble_semantic_symbols()
    -> Result<()> {
        let combined = vec![
            OutputCombinedSymbol::Text("x".to_string()),
            OutputCombinedSymbol::TagLeftAngle,
            OutputCombinedSymbol::TagAttributeValue(AttributeValue::Text("v".to_string())),
        ];

        assert!(
            assemble_semantic_symbols(combined)
                .is_err_and(|error| error.to_string().contains("Attribute value without name"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_attribute_value_with_no_open_tag() -> Result<()> {
        let combined = vec![OutputCombinedSymbol::TagAttributeValue(
            AttributeValue::Text("v".to_string()),
        )];

        assert!(
            assemble_semantic_symbols(combined)
                .is_err_and(|error| error.to_string().contains("Unexpected tag attribute value"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unexpected_self_close_with_no_open_tag() -> Result<()> {
        let state = make_state(vec![OutputSymbol::TagSelfClose]);

        assert!(
            combine_output_symbols(&state)
                .is_err_and(|error| error.to_string().contains("Unexpected self-closing tag"))
        );

        Ok(())
    }

    #[test]
    fn merge_adjacent_symbols_collapses_consecutive_same_kind_tokens() -> Result<()> {
        let combined = merge_adjacent_symbols(&make_state(vec![
            OutputSymbol::Text("a".to_string()),
            OutputSymbol::Text("b".to_string()),
            OutputSymbol::TagLeftAnglePlusWhitespace,
            OutputSymbol::TagLeftAnglePlusWhitespace,
            OutputSymbol::TagCloseBeforeNamePlusWhitespace("".to_string()),
            OutputSymbol::TagCloseBeforeNamePlusWhitespace("".to_string()),
            OutputSymbol::TagName("d".to_string()),
            OutputSymbol::TagName("iv".to_string()),
            OutputSymbol::TagPadding,
            OutputSymbol::TagPadding,
            OutputSymbol::TagAttributeName("c".to_string()),
            OutputSymbol::TagAttributeName("lass".to_string()),
        ]))
        .unwrap_or_default();

        let expected_discriminants = [
            discriminant(&OutputCombinedSymbol::Text(String::new())),
            discriminant(&OutputCombinedSymbol::TagLeftAngle),
            discriminant(&OutputCombinedSymbol::TagCloseBeforeName),
            discriminant(&OutputCombinedSymbol::TagName(String::new())),
            discriminant(&OutputCombinedSymbol::TagPadding),
            discriminant(&OutputCombinedSymbol::TagAttributeName(String::new())),
        ];

        assert_eq!(combined.len(), expected_discriminants.len());

        for (actual, expected) in combined.iter().zip(expected_discriminants.iter()) {
            assert_eq!(discriminant(actual), *expected);
        }

        assert!(matches!(&combined[0], OutputCombinedSymbol::Text(text) if text == "ab"));
        assert!(matches!(&combined[3], OutputCombinedSymbol::TagName(name) if name == "div"));
        assert!(matches!(
            &combined[5],
            OutputCombinedSymbol::TagAttributeName(name) if name == "class"
        ));

        Ok(())
    }

    #[test]
    fn assemble_semantic_symbols_merges_consecutive_text() -> Result<()> {
        let combined = vec![
            OutputCombinedSymbol::Text("a".to_string()),
            OutputCombinedSymbol::Text("b".to_string()),
        ];

        assert!(
            assemble_semantic_symbols(combined).is_ok_and(|mut semantic| {
                let first = semantic.pop_front();

                matches!(first, Some(OutputSemanticSymbol::Text(text)) if text == "ab")
                    && semantic.is_empty()
            })
        );

        Ok(())
    }
}
