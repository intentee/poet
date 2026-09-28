use markdown::mdast::Heading;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Text;

use crate::mdast_container_children::mdast_container_children;

#[must_use]
pub fn find_text_content_in_mdast(mdast: &Node) -> String {
    match mdast {
        Node::Heading(Heading { children, .. }) | Node::Paragraph(Paragraph { children, .. }) => {
            children.iter().map(find_text_content_in_mdast).collect()
        }
        Node::Text(Text { value, .. }) => value.clone(),
        other_node => mdast_container_children(other_node)
            .map(|children| children.iter().map(find_text_content_in_mdast).collect())
            .unwrap_or_default(),
    }
}
