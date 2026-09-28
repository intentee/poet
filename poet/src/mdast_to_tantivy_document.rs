use std::sync::Arc;

use markdown::mdast::Heading as MdastHeading;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Text;
use poet_mdx::mdast_container_children::mdast_container_children;
use tantivy::TantivyDocument;

use crate::search_index_fields::SearchIndexFields;

enum ParentElementType {
    Heading,
    Other,
    Paragraph,
}

fn traverse_mdast_children(
    document: &mut TantivyDocument,
    children: &[Node],
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
        Node::Heading(MdastHeading { children, .. }) => {
            traverse_mdast_children(document, children, fields, &ParentElementType::Heading);
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            traverse_mdast_children(document, children, fields, &ParentElementType::Paragraph);
        }
        Node::Text(Text { value, .. }) => match parent_element_type {
            ParentElementType::Heading => {
                document.add_field_value(fields.header, value);
            }
            ParentElementType::Paragraph => {
                document.add_field_value(fields.paragraph, value);
            }
            ParentElementType::Other => {}
        },
        other_node => traverse_mdast_children(
            document,
            mdast_container_children(other_node).map_or(&[], Vec::as_slice),
            fields,
            parent_element_type,
        ),
    }
}

pub fn mdast_to_tantivy_document(fields: Arc<SearchIndexFields>, mdast: &Node) -> TantivyDocument {
    let mut document = TantivyDocument::new();

    traverse_mdast(&mut document, mdast, fields, &ParentElementType::Other);

    document
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use markdown::mdast::Root;
    use poet_mdx::string_to_mdast::string_to_mdast;
    use tantivy::schema::Field;
    use tantivy::schema::Value as _;

    use super::*;
    use crate::search_index_schema::SearchIndexSchema;

    fn field_text(document: &TantivyDocument, field: Field) -> Option<String> {
        document
            .get_first(field)
            .and_then(|value| value.as_str())
            .map(|text| text.to_string())
    }

    #[test]
    fn routes_heading_text_to_header_and_paragraph_text_to_paragraph() -> Result<()> {
        let fields = Arc::new(SearchIndexSchema::default().fields);
        let mdast = string_to_mdast("# Title\n\nBody paragraph")?;
        let document = mdast_to_tantivy_document(fields.clone(), &mdast);

        assert_eq!(
            field_text(&document, fields.header),
            Some("Title".to_string())
        );
        assert_eq!(
            field_text(&document, fields.paragraph),
            Some("Body paragraph".to_string())
        );

        Ok(())
    }

    #[test]
    fn does_not_index_text_outside_heading_or_paragraph() {
        let fields = Arc::new(SearchIndexSchema::default().fields);
        let mdast = Node::Root(Root {
            children: vec![Node::Text(Text {
                value: "loose".to_string(),
                position: None,
            })],
            position: None,
        });
        let document = mdast_to_tantivy_document(fields.clone(), &mdast);

        assert_eq!(field_text(&document, fields.header), None);
        assert_eq!(field_text(&document, fields.paragraph), None);
    }
}
