use anyhow::Result;
use markdown::mdast::Node;
use markdown::mdast::Root;
use markdown::mdast::Toml;
use serde::de::DeserializeOwned;

pub fn find_front_matter_in_mdast<TFrontMatter: DeserializeOwned>(
    mdast: &Node,
) -> Result<Option<TFrontMatter>> {
    match mdast {
        Node::Root(Root { children, .. }) => {
            for child in children {
                if let Some(front_matter) = find_front_matter_in_mdast::<TFrontMatter>(child)? {
                    return Ok(Some(front_matter));
                }
            }

            Ok(None)
        }
        Node::Toml(Toml { value, .. }) => Ok(Some(toml::from_str::<TFrontMatter>(value)?)),
        _ => Ok(None),
    }
}
