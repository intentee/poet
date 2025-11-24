use std::collections::VecDeque;

use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::output_semantic_symbol::OutputSemanticSymbol;
use super::tag::Tag;
use super::tag_stack_node::TagStackNode;

pub fn combine_tag_stack(
    current_node: &mut TagStackNode,
    opened_tags: &mut VecDeque<Tag>,
    semantic_symbols: &mut VecDeque<OutputSemanticSymbol>,
) -> Result<(), ParseError> {
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

                    combine_tag_stack(current_node, opened_tags, semantic_symbols)
                }
                Some(OutputSemanticSymbol::Tag(tag)) => {
                    if tag.is_closing {
                        if let Some(opening_tag) = &opening_tag {
                            if opening_tag.tag_name.name != tag.tag_name.name {
                                return Err(LexError::UnexpectedInput(format!(
                                    "Mismatched closing tag: expected </{}>, got </{}>",
                                    opening_tag.tag_name.name, tag.tag_name.name
                                ))
                                .into_err(Position::NONE));
                            }

                            opened_tags.pop_back();
                        } else {
                            return Err(LexError::UnexpectedInput(format!(
                                "Unexpected closing tag: </{}>",
                                tag.tag_name.name
                            ))
                            .into_err(Position::NONE));
                        }

                        *is_closed = true;

                        Ok(())
                    } else if tag.is_self_closing || tag.tag_name.is_void_element() {
                        children.push(TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        });

                        combine_tag_stack(current_node, opened_tags, semantic_symbols)
                    } else {
                        opened_tags.push_back(tag.clone());

                        let mut child_node = TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        };

                        combine_tag_stack(&mut child_node, opened_tags, semantic_symbols)?;

                        children.push(child_node);

                        combine_tag_stack(current_node, opened_tags, semantic_symbols)
                    }
                }
                Some(OutputSemanticSymbol::Text(text)) => {
                    if !text.is_empty() {
                        children.push(TagStackNode::Text(text));
                    }

                    combine_tag_stack(current_node, opened_tags, semantic_symbols)
                }
                None => {
                    if !opened_tags.is_empty() {
                        return Err(LexError::UnexpectedInput(format!(
                            "Unclosed tag: <{}>",
                            match opened_tags.back() {
                                Some(tag) => &tag.tag_name.name,
                                None =>
                                    return Err(LexError::UnexpectedInput(
                                        "No opened tags found".to_string(),
                                    )
                                    .into_err(Position::NONE)),
                            }
                        ))
                        .into_err(Position::NONE));
                    }

                    Ok(())
                }
            }
        }
        TagStackNode::BodyExpression(_) => Err(LexError::UnexpectedInput(
            "Cannot add child to body expression node".to_string(),
        )
        .into_err(Position::NONE)),
        TagStackNode::Text(_) => Err(LexError::UnexpectedInput(
            "Cannot add child to text node".to_string(),
        )
        .into_err(Position::NONE)),
    }
}
