mod attribute;
mod attribute_value;
mod escape_html;
mod expression_collection;
mod expression_reference;
mod output_combined_symbol;
mod output_semantic_symbol;
mod output_symbol;
mod parser_state;
mod tag;
mod tag_stack_node;

use std::collections::VecDeque;

use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;
use rhai::ImmutableString;
use rhai::LexError;
use rhai::ParseError;
use rhai::ParseErrorType;
use rhai::Position;

use self::attribute::Attribute;
use self::attribute_value::AttributeValue;
use self::escape_html::escape_html;
use self::expression_collection::ExpressionCollection;
use self::expression_reference::ExpressionReference;
use self::output_combined_symbol::OutputCombinedSymbol;
use self::output_semantic_symbol::OutputSemanticSymbol;
use self::output_symbol::OutputSymbol;
use self::parser_state::ParserState;
use self::tag::Tag;
use self::tag_stack_node::TagStackNode;

pub fn parse_component(
    symbols: &[ImmutableString],
    state: &mut Dynamic,
) -> Result<Option<ImmutableString>, ParseError> {
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
                // This is where the expression ends, so lets optimize the internal state now
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

pub fn render_tag(
    context: &mut EvalContext,
    expression_collection: &mut ExpressionCollection,
    tag: &Tag,
) -> Result<String, Box<EvalAltResult>> {
    let mut result = String::new();

    if tag.is_closing {
        result.push_str("</");
        result.push_str(&tag.name);
        result.push('>');

        return Ok(result);
    }

    result.push('<');
    result.push_str(&tag.name);

    for attribute in &tag.attributes {
        result.push(' ');
        result.push_str(&attribute.name);

        if let Some(value) = &attribute.value {
            result.push('=');
            result.push('"');
            match value {
                AttributeValue::Expression(expression_reference) => {
                    result.push_str(&escape_html(
                        &expression_collection.render_expression(context, expression_reference)?,
                    ));
                }
                AttributeValue::Text(text) => {
                    result.push_str(text);
                }
            };
            result.push('"');
        }
    }

    if tag.is_self_closing {
        result.push_str(" />");
    } else {
        result.push('>');
    }

    Ok(result)
}

pub fn eval_component(
    context: &mut EvalContext,
    inputs: &[Expression],
    state: &Dynamic,
) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut expression_collection = ExpressionCollection {
        expressions: inputs.to_vec(),
    };
    // let mut inputs_deque: VecDeque<&Expression> = inputs.iter().collect();

    // let mut shift_expression_tree = || {
    //     if let Some(expression) = inputs_deque.pop_front() {
    //         Ok(context.eval_expression_tree(expression)?.into_string()?)
    //     } else {
    //         Err(Box::new(EvalAltResult::ErrorParsing(
    //             ParseErrorType::BadInput(LexError::UnexpectedInput(format!(
    //                 "Exprected expression after component block (got nothing)"
    //             ))),
    //             Position::NONE,
    //         )))
    //     }
    // };

    let mut expression_index = 0;
    let mut combined_symbols: Vec<OutputCombinedSymbol> = vec![];

    for node in state.as_array_ref()?.iter() {
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
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Attribute value expression without name".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
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
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Attribute value expression without name".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
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
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        format!("Unexpected tag opening after {last_symbol:?}"),
                        Vec::new(),
                        Position::NONE,
                    )));
                }
            },
            OutputCombinedSymbol::TagCloseBeforeName => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { is_closing, .. })) => {
                    *is_closing = true;
                }
                _ => {
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Unexpected tag closing".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
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
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Unexpected tag name".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
                }
            },
            OutputCombinedSymbol::TagAttributeName(name) => match semantic_symbols.back_mut() {
                Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                    attributes.push(Attribute { name, value: None });
                }
                _ => {
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Unexpected tag attribute name".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
                }
            },
            OutputCombinedSymbol::TagAttributeValue(attribute_value) => {
                match semantic_symbols.back_mut() {
                    Some(OutputSemanticSymbol::Tag(Tag { attributes, .. })) => {
                        if let Some(last_attribute) = attributes.last_mut() {
                            last_attribute.value = Some(attribute_value);
                        } else {
                            return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                                "Attribute value without name".to_string(),
                                Vec::new(),
                                Position::NONE,
                            )));
                        }
                    }
                    _ => {
                        return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                            "Unexpected tag attribute value".to_string(),
                            Vec::new(),
                            Position::NONE,
                        )));
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
                    return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                        "Unexpected self-closing tag".to_string(),
                        Vec::new(),
                        Position::NONE,
                    )));
                }
            },
            OutputCombinedSymbol::TagRightAngle => {}
        }
    }

    let mut tag_stack = TagStackNode::Tag {
        children: vec![],
        is_closed: false,
        opening_tag: None,
    };

    build_tag_stack(&mut tag_stack, &mut semantic_symbols)?;

    let rendered_tag_stack = render_tag_stack(context, &tag_stack, &mut expression_collection)?;

    Ok(Dynamic::from(rendered_tag_stack))
}

fn build_tag_stack<'node>(
    current_node: &'node mut TagStackNode,
    semantic_symbols: &mut VecDeque<OutputSemanticSymbol>,
) -> Result<(), Box<EvalAltResult>> {
    match current_node {
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let next_symbol = semantic_symbols.pop_front();

            match next_symbol {
                Some(OutputSemanticSymbol::BodyExpression(expression_reference)) => {
                    children.push(TagStackNode::BodyExpression(expression_reference));

                    build_tag_stack(current_node, semantic_symbols)
                }
                Some(OutputSemanticSymbol::Tag(tag)) => {
                    if tag.is_closing {
                        if let Some(opening_tag) = &opening_tag {
                            if opening_tag.name != tag.name {
                                return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                                    format!(
                                        "Mismatched closing tag: expected </{}>, got </{}>",
                                        opening_tag.name, tag.name
                                    ),
                                    Vec::new(),
                                    Position::NONE,
                                )));
                            }
                        } else {
                            return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                                format!("Unexpected closing tag: </{}>", tag.name),
                                Vec::new(),
                                Position::NONE,
                            )));
                        }

                        *is_closed = true;

                        Ok(())
                    } else if tag.is_self_closing {
                        children.push(TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        });

                        build_tag_stack(current_node, semantic_symbols)
                    } else {
                        let mut child_node = TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        };

                        build_tag_stack(&mut child_node, semantic_symbols)?;

                        children.push(child_node);

                        build_tag_stack(current_node, semantic_symbols)
                    }
                }
                Some(OutputSemanticSymbol::Text(text)) => {
                    if !text.is_empty() {
                        children.push(TagStackNode::Text(text));
                    }

                    build_tag_stack(current_node, semantic_symbols)
                }
                None => Ok(()),
            }
        }
        TagStackNode::BodyExpression(_) => {
            return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                "Cannot add child to body expression node".to_string(),
                Vec::new(),
                Position::NONE,
            )));
        }
        TagStackNode::Text(_) => {
            return Err(Box::new(EvalAltResult::ErrorCustomSyntax(
                "Cannot add child to text node".to_string(),
                Vec::new(),
                Position::NONE,
            )));
        }
    }
}

fn render_tag_stack<'node>(
    context: &mut EvalContext,
    current_node: &'node TagStackNode,
    expression_collection: &mut ExpressionCollection,
) -> Result<String, Box<EvalAltResult>> {
    match current_node {
        TagStackNode::BodyExpression(expression_reference) => {
            Ok(expression_collection.render_expression(context, expression_reference)?)
        }
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let mut result = String::new();

            if let Some(opening_tag) = &opening_tag
                && !opening_tag.is_component()
            {
                result.push_str(&render_tag(context, expression_collection, opening_tag)?);
            }

            for child in children {
                result.push_str(&render_tag_stack(context, child, expression_collection)?);
            }

            if let Some(opening_tag) = &opening_tag
                && *is_closed
                && !opening_tag.is_component()
            {
                result.push_str(&format!("</{}>", opening_tag.name));
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.is_component()
            {
                // context.global_runtime_state_mut().iter_imports().for_each(|(name, module)| {
                //     println!("imported module: {} {:#?}", name, module);
                // });
                context.iter_namespaces().for_each(|module| {
                    println!("regsitered namespace: {:#?}", module);
                });

                for (name, is_const, dynamic) in context.scope().iter() {
                    println!("scoped variable: {} {:#?} = {:#?}", name, is_const, dynamic);
                }

                // println!("Eval result: {:#?}", context.engine().eval::<Dynamic>("Note::template(1, 2, 3)")?);

                // context.call_fn(
                //     "template",
                //     (Dynamic::from(""), Dynamic::from(""), Dynamic::from("")),
                // )?;
            }

            Ok(result)
        }
        TagStackNode::Text(text) => Ok(text.clone()),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Engine;

    use super::*;

    #[test]
    fn test_docs_parser() -> Result<()> {
        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
        // engine.set_strict_variables(true);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            eval_component,
        );

        // engine.register_static_module("LayoutHomepage", engine.module_resolver().resolve(
        //     &engine,
        //     None,
        //     "shortcodes/LayoutHomepage.rhai",
        //     Position::NONE,
        // )?);
        //
        // let note_module = engine.module_resolver().resolve(
        //     &engine,
        //     None,
        //     "shortcodes/Note.rhai",
        //     Position::NONE,
        // )?;
        //
        // for signature in note_module.gen_fn_signatures_with_mapper(|name| format!("Note::{name}").into()) {
        //     println!("Function signature: {:#?}", signature);
        // }
        //
        // println!("Note::template function: {:#?}", note_module.get_script_fn("template", 3));

        // engine.register_static_module("Note", note_module);

        println!(
            "{}",
            engine.eval::<String>(
                r#"
            fn MyComponent(context, content, props) {
                context.assets.add("resouces/controller_foo.tsx");

                component {
                    <!DOCTYPE html>
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
                            {" - "}

                            <Note>
                                {if content.is_empty() {
                                    component {
                                        <div>
                                            NOTE EMPTY CONTENT
                                        </div>
                                    }
                                } else {
                                    content
                                }}
                            </Note>
                        </button>
                    </LayoutHomepage>
                }
            }

            MyComponent(#{
                render: || "wow",
                assets: #{
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
