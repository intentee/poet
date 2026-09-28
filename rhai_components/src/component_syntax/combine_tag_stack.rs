use std::collections::VecDeque;

use rhai::LexError;
use rhai::ParseError;
use rhai::Position;

use super::output_semantic_symbol::OutputSemanticSymbol;
use super::tag::Tag;
use super::tag_kind::TagKind;
use super::tag_stack::TagStack;
use super::tag_stack_element::TagStackElement;
use super::tag_stack_node::TagStackNode;

fn combine_element_children(
    opening_tag: &Tag,
    semantic_symbols: &mut VecDeque<OutputSemanticSymbol>,
) -> Result<Vec<TagStackNode>, ParseError> {
    let mut children = Vec::new();

    loop {
        match semantic_symbols.pop_front() {
            None => {
                return Err(LexError::UnexpectedInput(format!(
                    "Unclosed tag: <{}>",
                    opening_tag.tag_name.name
                ))
                .into_err(Position::NONE));
            }
            Some(OutputSemanticSymbol::Tag(tag)) if tag.kind == TagKind::Closing => {
                if tag.tag_name.name == opening_tag.tag_name.name {
                    return Ok(children);
                }

                return Err(LexError::UnexpectedInput(format!(
                    "Mismatched closing tag: expected </{}>, got </{}>",
                    opening_tag.tag_name.name, tag.tag_name.name
                ))
                .into_err(Position::NONE));
            }
            Some(semantic_symbol) => {
                append_semantic_symbol(&mut children, semantic_symbol, semantic_symbols)?;
            }
        }
    }
}

fn append_semantic_symbol(
    children: &mut Vec<TagStackNode>,
    semantic_symbol: OutputSemanticSymbol,
    semantic_symbols: &mut VecDeque<OutputSemanticSymbol>,
) -> Result<(), ParseError> {
    match semantic_symbol {
        OutputSemanticSymbol::BodyExpression(expression_reference) => {
            children.push(TagStackNode::BodyExpression(expression_reference));
        }
        OutputSemanticSymbol::Tag(opening_tag) => {
            let element_children = if opening_tag.kind == TagKind::Opening
                && !opening_tag.tag_name.is_void_element()
            {
                combine_element_children(&opening_tag, semantic_symbols)?
            } else {
                Vec::new()
            };

            children.push(TagStackNode::Element(TagStackElement {
                children: element_children,
                opening_tag,
            }));
        }
        OutputSemanticSymbol::Text(text) => {
            if !text.is_empty() {
                children.push(TagStackNode::Text(text));
            }
        }
    }

    Ok(())
}

pub fn combine_tag_stack(
    semantic_symbols: &mut VecDeque<OutputSemanticSymbol>,
) -> Result<TagStack, ParseError> {
    let mut children = Vec::new();

    while let Some(semantic_symbol) = semantic_symbols.pop_front() {
        match semantic_symbol {
            OutputSemanticSymbol::Tag(tag) if tag.kind == TagKind::Closing => {
                return Err(LexError::UnexpectedInput(format!(
                    "Unexpected closing tag: </{}>",
                    tag.tag_name.name
                ))
                .into_err(Position::NONE));
            }
            semantic_symbol => {
                append_semantic_symbol(&mut children, semantic_symbol, semantic_symbols)?;
            }
        }
    }

    Ok(TagStack { children })
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use rhai::LexError;
    use rhai::ParseError;
    use rhai::ParseErrorType;

    use super::OutputSemanticSymbol;
    use super::Tag;
    use super::TagKind;
    use super::TagStackNode;
    use super::combine_tag_stack;
    use crate::component_syntax::tag_name::TagName;

    fn tag(name: &str, kind: TagKind) -> OutputSemanticSymbol {
        OutputSemanticSymbol::Tag(Tag {
            attributes: vec![],
            kind,
            tag_name: TagName {
                name: name.to_owned(),
            },
        })
    }

    fn is_unexpected_input(parse_error: &ParseError, expected_message: &str) -> bool {
        matches!(
            parse_error.err_type(),
            ParseErrorType::BadInput(LexError::UnexpectedInput(message)) if message == expected_message
        )
    }

    #[test]
    fn rejects_mismatched_closing_tag() {
        let mut semantic_symbols =
            VecDeque::from([tag("div", TagKind::Opening), tag("span", TagKind::Closing)]);

        assert!(
            combine_tag_stack(&mut semantic_symbols).is_err_and(|parse_error| {
                is_unexpected_input(
                    &parse_error,
                    "Mismatched closing tag: expected </div>, got </span>",
                )
            })
        );
    }

    #[test]
    fn rejects_closing_tag_without_open_tag() {
        let mut semantic_symbols = VecDeque::from([tag("div", TagKind::Closing)]);

        assert!(
            combine_tag_stack(&mut semantic_symbols).is_err_and(|parse_error| {
                is_unexpected_input(&parse_error, "Unexpected closing tag: </div>")
            })
        );
    }

    #[test]
    fn rejects_unclosed_tag() {
        let mut semantic_symbols = VecDeque::from([tag("div", TagKind::Opening)]);

        assert!(
            combine_tag_stack(&mut semantic_symbols)
                .is_err_and(|parse_error| is_unexpected_input(&parse_error, "Unclosed tag: <div>"))
        );
    }

    #[test]
    fn adds_void_element_without_requiring_closing_tag() {
        let mut semantic_symbols = VecDeque::from([tag("br", TagKind::Opening)]);

        assert!(
            combine_tag_stack(&mut semantic_symbols).is_ok_and(|tag_stack| {
                matches!(
                    tag_stack.children.as_slice(),
                    [TagStackNode::Element(element)]
                        if element.opening_tag.tag_name.name == "br" && element.children.is_empty()
                )
            })
        );
    }

    #[test]
    fn nests_children_until_matching_closing_tag() {
        let mut semantic_symbols = VecDeque::from([
            tag("ul", TagKind::Opening),
            tag("li", TagKind::SelfClosing),
            tag("ul", TagKind::Closing),
        ]);

        assert!(combine_tag_stack(&mut semantic_symbols).is_ok_and(|tag_stack| {
            matches!(
                tag_stack.children.as_slice(),
                [TagStackNode::Element(element)]
                    if matches!(
                        element.children.as_slice(),
                        [TagStackNode::Element(child)] if child.opening_tag.tag_name.name == "li"
                    )
            )
        }));
    }

    #[test]
    fn drops_empty_text() {
        let mut semantic_symbols = VecDeque::from([OutputSemanticSymbol::Text(String::new())]);

        assert!(
            combine_tag_stack(&mut semantic_symbols)
                .is_ok_and(|tag_stack| tag_stack.children.is_empty())
        );
    }

    #[test]
    fn rejects_mismatched_closing_tag_inside_nested_element() {
        let mut semantic_symbols = VecDeque::from([
            tag("ul", TagKind::Opening),
            tag("li", TagKind::Opening),
            tag("ul", TagKind::Closing),
        ]);

        assert!(
            combine_tag_stack(&mut semantic_symbols).is_err_and(|parse_error| {
                is_unexpected_input(
                    &parse_error,
                    "Mismatched closing tag: expected </li>, got </ul>",
                )
            })
        );
    }
}
