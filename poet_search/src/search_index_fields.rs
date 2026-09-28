use markdown::mdast::Heading;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Text;
use poet_content::content_document_source::ContentDocumentSource;
use poet_mdx::mdast_container_children::mdast_container_children;
use tantivy::Index;
use tantivy::TantivyDocument;
use tantivy::query::QueryParser;
use tantivy::schema::Field;

use crate::mdast_text_parent::MdastTextParent;

#[derive(Clone, Copy)]
pub struct SearchIndexFields {
    pub basename: Field,
    pub description: Field,
    pub header: Field,
    pub paragraph: Field,
    pub title: Field,
}

impl SearchIndexFields {
    #[must_use]
    pub fn query_parser(&self, index: &Index) -> QueryParser {
        let mut query_parser = QueryParser::for_index(
            index,
            vec![self.title, self.description, self.header, self.paragraph],
        );

        query_parser.set_field_boost(self.title, 4.0);
        query_parser.set_field_boost(self.description, 3.0);
        query_parser.set_field_boost(self.header, 2.0);

        query_parser
    }

    #[must_use]
    pub fn tantivy_document(
        &self,
        ContentDocumentSource {
            mdast, reference, ..
        }: &ContentDocumentSource,
    ) -> TantivyDocument {
        let mut document = TantivyDocument::new();

        document.add_text(self.basename, reference.basename().0);
        document.add_text(self.title, &reference.front_matter.title);
        document.add_text(self.description, &reference.front_matter.description);

        self.add_mdast_text(&mut document, mdast, MdastTextParent::Other);

        document
    }

    fn add_mdast_children_text(
        &self,
        document: &mut TantivyDocument,
        children: &[Node],
        parent: MdastTextParent,
    ) {
        for child in children {
            self.add_mdast_text(document, child, parent);
        }
    }

    fn add_mdast_text(
        &self,
        document: &mut TantivyDocument,
        mdast: &Node,
        parent: MdastTextParent,
    ) {
        match mdast {
            Node::Heading(Heading { children, .. }) => {
                self.add_mdast_children_text(document, children, MdastTextParent::Heading);
            }
            Node::Paragraph(Paragraph { children, .. }) => {
                self.add_mdast_children_text(document, children, MdastTextParent::Paragraph);
            }
            Node::Text(Text { value, .. }) => match parent {
                MdastTextParent::Heading => document.add_text(self.header, value),
                MdastTextParent::Other => {}
                MdastTextParent::Paragraph => document.add_text(self.paragraph, value),
            },
            other_node => self.add_mdast_children_text(
                document,
                mdast_container_children(other_node).map_or(&[], Vec::as_slice),
                parent,
            ),
        }
    }
}
