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
use tantivy::TantivyDocument;

use crate::search_index_fields::SearchIndexFields;

fn find_headings_in_mdast(mdast: &Node) {
    match mdast {
        Node::Blockquote(Blockquote { children, .. })
        | Node::Delete(Delete { children, .. })
        | Node::Emphasis(Emphasis { children, .. })
        | Node::Heading(MdastHeading { children, .. })
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
        | Node::TableRow(TableRow { children, .. }) => {}
        _ => {}
    }
}

pub fn mdast_to_tantivy_document(fields: Arc<SearchIndexFields>, mdast: &Node) -> TantivyDocument {
    let document = TantivyDocument::new();

    find_headings_in_mdast(mdast);

    document
}
