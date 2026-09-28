use markdown::mdast::Node;
use slug::slugify;

use poet_mdx::find_text_content_in_mdast::find_text_content_in_mdast;

pub fn mdast_children_to_heading_id(children: &[Node]) -> String {
    slugify(
        children
            .iter()
            .map(find_text_content_in_mdast)
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use markdown::mdast::Text;

    use super::*;

    fn text_nodes(values: &[&str]) -> Vec<Node> {
        values
            .iter()
            .map(|value| {
                Node::Text(Text {
                    value: value.to_string(),
                    position: None,
                })
            })
            .collect()
    }

    #[test]
    fn concatenates_child_text_before_slugifying() {
        assert_eq!(
            mdast_children_to_heading_id(&text_nodes(&["Hello ", "World"])),
            "hello-world"
        );
    }
}
