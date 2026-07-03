use anyhow::Result;
use markdown::mdast::Node;
use slug::slugify;

use crate::find_text_content_in_mdast::find_text_content_in_mdast;

pub fn mdast_children_to_heading_id(children: &Vec<Node>) -> Result<String> {
    let mut inner_text = String::new();

    for child in children {
        inner_text.push_str(&find_text_content_in_mdast(child)?);
    }

    Ok(slugify(inner_text))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
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
    fn concatenates_child_text_before_slugifying() -> Result<()> {
        assert_eq!(
            mdast_children_to_heading_id(&text_nodes(&["Hello ", "World"]))?,
            "hello-world"
        );

        Ok(())
    }
}
