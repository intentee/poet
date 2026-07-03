use anyhow::Result;
use markdown::mdast::Blockquote;
use markdown::mdast::Delete;
use markdown::mdast::Emphasis;
use markdown::mdast::Heading;
use markdown::mdast::Link;
use markdown::mdast::List;
use markdown::mdast::ListItem;
use markdown::mdast::MdxJsxFlowElement;
use markdown::mdast::MdxJsxTextElement;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Root;
use markdown::mdast::Strong;
use markdown::mdast::Table;
use markdown::mdast::TableCell;
use markdown::mdast::TableRow;
use markdown::mdast::Text;

pub fn find_text_content_in_mdast(mdast: &Node) -> Result<String> {
    let mut result = String::new();

    match mdast {
        Node::Blockquote(Blockquote { children, .. })
        | Node::Delete(Delete { children, .. })
        | Node::Emphasis(Emphasis { children, .. })
        | Node::Heading(Heading { children, .. })
        | Node::Link(Link { children, .. })
        | Node::List(List { children, .. })
        | Node::ListItem(ListItem { children, .. })
        | Node::MdxJsxFlowElement(MdxJsxFlowElement { children, .. })
        | Node::MdxJsxTextElement(MdxJsxTextElement { children, .. })
        | Node::Paragraph(Paragraph { children, .. })
        | Node::Root(Root { children, .. })
        | Node::Strong(Strong { children, .. })
        | Node::Table(Table { children, .. })
        | Node::TableCell(TableCell { children, .. })
        | Node::TableRow(TableRow { children, .. }) => {
            for child in children {
                result.push_str(&find_text_content_in_mdast(child)?);
            }
        }
        Node::Text(Text { value, .. }) => {
            result.push_str(value);
        }
        _ => {}
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::string_to_mdast::string_to_mdast;

    #[test]
    fn concatenates_text_across_nested_inline_nodes() -> Result<()> {
        let mdast = string_to_mdast("Hello **bold** and *italic*")?;

        assert_eq!(find_text_content_in_mdast(&mdast)?, "Hello bold and italic");

        Ok(())
    }

    #[test]
    fn skips_non_text_leaf_nodes() -> Result<()> {
        let mdast = string_to_mdast("text `code` more")?;

        assert_eq!(find_text_content_in_mdast(&mdast)?, "text  more");

        Ok(())
    }

    #[test]
    fn collects_text_across_block_container_nodes() -> Result<()> {
        let mdast = string_to_mdast(
            "# Heading text\n\n> quote text\n\n- list text\n\n[link text](https://example.com)\n\n| cell text |\n| --- |\n| data text |",
        )?;

        let text = find_text_content_in_mdast(&mdast)?;

        for fragment in [
            "Heading text",
            "quote text",
            "list text",
            "link text",
            "cell text",
            "data text",
        ] {
            assert!(text.contains(fragment), "missing fragment: {fragment}");
        }

        Ok(())
    }
}
