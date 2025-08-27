use anyhow::Result;
use markdown::mdast::Node;
use markdown::mdast::Root;
use markdown::mdast::Toml;

use crate::front_matter::FrontMatter;

pub fn find_front_matter_in_mdast(mdast: &Node) -> Result<Option<FrontMatter>> {
    match mdast {
        Node::Root(Root { children, .. }) => {
            for child in children {
                if let Some(front_matter) = find_front_matter_in_mdast(child)? {
                    return Ok(Some(front_matter));
                }
            }

            Ok(None)
        }
        Node::Toml(Toml { value, .. }) => Ok(Some(toml::from_str::<FrontMatter>(value)?)),
        _ => Ok(None),
    }
}
