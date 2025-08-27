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

pub fn combine_output_symbols(
    state: &Dynamic,
) -> Result<VecDeque<OutputSemanticSymbol>, ParseError> {
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
        match node.clone().try_cast::<OutputSymbol>().unwrap() {
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
            OutputSymbol::TagLeftAnglePlusWhitespace(_) => match combined_symbols.last_mut() {
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
            OutputSymbol::TagContent(_) => {}
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

    let mut semantic_symbols: VecDeque<OutputSemanticSymbol> = VecDeque::new();

    for output_combined_symbol in combined_symbols {
        match output_combined_symbol {
            OutputCombinedSymbol::BodyExpression(expression_reference) => {
                semantic_symbols
                    .push_back(OutputSemanticSymbol::BodyExpression(expression_reference));
            }
            OutputCombinedSymbol::Text(text) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Text(existing_text)) => {
                    existing_text.push_str(&text.trim());
                }
                _ => {
                    semantic_symbols.push_back(OutputSemanticSymbol::Text(text.trim().to_string()));
                }
            },
            OutputCombinedSymbol::TagLeftAngle => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(_)) | Some(OutputSemanticSymbol::Text(_)) => {
                    semantic_symbols.push_back(OutputSemanticSymbol::Tag(Tag {
                        attributes: vec![],
                        is_closing: false,
                        is_self_closing: false,
                        name: String::new(),
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
                    name: existing_name,
                    ..
                })) => {
                    *existing_name = name;
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
