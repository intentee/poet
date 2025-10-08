use std::sync::Arc;

use markdown::mdast::Blockquote;
use markdown::mdast::Delete;
use markdown::mdast::Emphasis;
use markdown::mdast::Heading as MdastHeading;
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
use tantivy::TantivyDocument;

use crate::search_index_fields::SearchIndexFields;

enum ParentElementType {
    Heading,
    Other,
    Paragraph,
}

fn traverse_mdast_children(
    document: &mut TantivyDocument,
    children: &Vec<Node>,
    fields: Arc<SearchIndexFields>,
    parent_element_type: &ParentElementType,
) {
    for child in children {
        traverse_mdast(document, child, fields.clone(), parent_element_type);
    }
}

fn traverse_mdast(
    document: &mut TantivyDocument,
    mdast: &Node,
    fields: Arc<SearchIndexFields>,
    parent_element_type: &ParentElementType,
) {
    match mdast {
        Node::Blockquote(Blockquote { children, .. })
        | Node::Delete(Delete { children, .. })
        | Node::Emphasis(Emphasis { children, .. })
        | Node::Link(Link { children, .. })
        | Node::List(List { children, .. })
        | Node::ListItem(ListItem { children, .. })
        | Node::MdxJsxFlowElement(MdxJsxFlowElement { children, .. })
        | Node::MdxJsxTextElement(MdxJsxTextElement { children, .. })
        | Node::Root(Root { children, .. })
        | Node::Strong(Strong { children, .. })
        | Node::Table(Table { children, .. })
        | Node::TableCell(TableCell { children, .. })
        | Node::TableRow(TableRow { children, .. }) => {
            traverse_mdast_children(document, children, fields, parent_element_type);
        }
        Node::Heading(MdastHeading { children, .. }) => {
            traverse_mdast_children(document, children, fields, &ParentElementType::Heading);
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            traverse_mdast_children(document, children, fields, &ParentElementType::Paragraph);
        }
        Node::Text(Text { value, .. }) => {
            match parent_element_type {
                ParentElementType::Heading => {
                    document.add_field_value(fields.header, value);
                }
                ParentElementType::Paragraph => {
                    document.add_field_value(fields.paragraph, value);
                }
                ParentElementType::Other => {
                    // do not index other types of content
                }
            }
        }
        _ => {}
    }
}

pub fn mdast_to_tantivy_document(fields: Arc<SearchIndexFields>, mdast: &Node) -> TantivyDocument {
    let mut document = TantivyDocument::new();

    traverse_mdast(&mut document, mdast, fields, &ParentElementType::Other);

    document
}
