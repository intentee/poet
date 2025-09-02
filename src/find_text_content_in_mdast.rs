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
