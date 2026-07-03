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
                None => match opened_tags.back() {
                    Some(tag) => Err(LexError::UnexpectedInput(format!(
                        "Unclosed tag: <{}>",
                        tag.tag_name.name
                    ))
                    .into_err(Position::NONE)),
                    None => Ok(()),
                },
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

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use anyhow::Result;

    use super::OutputSemanticSymbol;
    use super::Tag;
    use super::TagStackNode;
    use super::combine_tag_stack;
    use crate::component_syntax::expression_reference::ExpressionReference;
    use crate::component_syntax::tag_name::TagName;

    fn make_tag(name: &str, is_closing: bool, is_self_closing: bool) -> Tag {
        Tag {
            attributes: vec![],
            is_closing,
            is_self_closing,
            tag_name: TagName {
                name: name.to_string(),
            },
        }
    }

    fn empty_root() -> TagStackNode {
        TagStackNode::Tag {
            children: vec![],
            is_closed: false,
            opening_tag: None,
        }
    }

    #[test]
    fn errs_when_root_node_is_body_expression() -> Result<()> {
        let mut root = TagStackNode::BodyExpression(ExpressionReference {
            expression_index: 0,
        });

        assert!(
            combine_tag_stack(&mut root, &mut VecDeque::new(), &mut VecDeque::new()).is_err_and(
                |error| error
                    .to_string()
                    .contains("Cannot add child to body expression node")
            )
        );

        Ok(())
    }

    #[test]
    fn errs_when_root_node_is_text() -> Result<()> {
        let mut root = TagStackNode::Text("hi".to_string());

        assert!(
            combine_tag_stack(&mut root, &mut VecDeque::new(), &mut VecDeque::new())
                .is_err_and(|error| error.to_string().contains("Cannot add child to text node"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_mismatched_closing_tag_name() -> Result<()> {
        let opening = make_tag("div", false, false);
        let mut root = TagStackNode::Tag {
            children: vec![],
            is_closed: false,
            opening_tag: Some(opening),
        };
        let mut symbols = VecDeque::new();

        symbols.push_back(OutputSemanticSymbol::Tag(make_tag("span", true, false)));

        assert!(
            combine_tag_stack(&mut root, &mut VecDeque::new(), &mut symbols)
                .is_err_and(|error| error.to_string().contains("Mismatched closing tag"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_closing_tag_when_no_tag_is_open() -> Result<()> {
        let mut root = empty_root();
        let mut symbols = VecDeque::new();

        symbols.push_back(OutputSemanticSymbol::Tag(make_tag("div", true, false)));

        assert!(
            combine_tag_stack(&mut root, &mut VecDeque::new(), &mut symbols)
                .is_err_and(|error| error.to_string().contains("Unexpected closing tag"))
        );

        Ok(())
    }

    #[test]
    fn errs_on_unclosed_tag_at_end() -> Result<()> {
        let mut root = empty_root();
        let mut opened = VecDeque::new();

        opened.push_back(make_tag("div", false, false));

        assert!(
            combine_tag_stack(&mut root, &mut opened, &mut VecDeque::new())
                .is_err_and(|error| error.to_string().contains("Unclosed tag"))
        );

        Ok(())
    }

    #[test]
    fn adds_void_element_as_child_without_requiring_close() -> Result<()> {
        let mut root = empty_root();
        let mut symbols = VecDeque::new();

        symbols.push_back(OutputSemanticSymbol::Tag(make_tag("br", false, false)));

        assert!(combine_tag_stack(&mut root, &mut VecDeque::new(), &mut symbols).is_ok());
        assert!(matches!(
            &root,
            TagStackNode::Tag { children, .. }
                if children.len() == 1
                && matches!(
                    &children[0],
                    TagStackNode::Tag { opening_tag: Some(tag), is_closed: false, .. }
                        if tag.tag_name.name == "br"
                )
        ));

        Ok(())
    }

    #[test]
    fn drops_empty_text_node_without_adding_child() -> Result<()> {
        let mut root = empty_root();
        let mut symbols = VecDeque::new();

        symbols.push_back(OutputSemanticSymbol::Text(String::new()));

        assert!(combine_tag_stack(&mut root, &mut VecDeque::new(), &mut symbols).is_ok());
        assert!(matches!(
            &root,
            TagStackNode::Tag { children, .. } if children.is_empty()
        ));

        Ok(())
    }
}
