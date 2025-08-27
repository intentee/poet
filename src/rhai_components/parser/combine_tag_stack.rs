use std::collections::VecDeque;

use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::output_semantic_symbol::OutputSemanticSymbol;
use super::tag_stack_node::TagStackNode;

pub fn combine_tag_stack<'node>(
    current_node: &'node mut TagStackNode,
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

                    combine_tag_stack(current_node, semantic_symbols)
                }
                Some(OutputSemanticSymbol::Tag(tag)) => {
                    if tag.is_closing {
                        if let Some(opening_tag) = &opening_tag {
                            if opening_tag.name != tag.name {
                                return Err(LexError::UnexpectedInput(format!(
                                    "Mismatched closing tag: expected </{}>, got </{}>",
                                    opening_tag.name, tag.name
                                ))
                                .into_err(Position::NONE));
                            }
                        } else {
                            return Err(LexError::UnexpectedInput(format!(
                                "Unexpected closing tag: </{}>",
                                tag.name
                            ))
                            .into_err(Position::NONE));
                        }

                        *is_closed = true;

                        Ok(())
                    } else if tag.is_self_closing {
                        children.push(TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        });

                        combine_tag_stack(current_node, semantic_symbols)
                    } else {
                        let mut child_node = TagStackNode::Tag {
                            children: vec![],
                            is_closed: false,
                            opening_tag: Some(tag),
                        };

                        combine_tag_stack(&mut child_node, semantic_symbols)?;

                        children.push(child_node);

                        combine_tag_stack(current_node, semantic_symbols)
                    }
                }
                Some(OutputSemanticSymbol::Text(text)) => {
                    if !text.is_empty() {
                        children.push(TagStackNode::Text(text));
                    }

                    combine_tag_stack(current_node, semantic_symbols)
                }
                None => Ok(()),
            }
        }
        TagStackNode::BodyExpression(_) => {
            return Err(LexError::UnexpectedInput(
                "Cannot add child to body expression node".to_string(),
            )
            .into_err(Position::NONE));
        }
        TagStackNode::Text(_) => {
            return Err(
                LexError::UnexpectedInput("Cannot add child to text node".to_string())
                    .into_err(Position::NONE),
            );
        }
    }
}
