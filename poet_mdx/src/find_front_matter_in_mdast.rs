use markdown::mdast::Node;
use markdown::mdast::Toml;
use serde::de::DeserializeOwned;

use crate::mdx_error::MdxError;

pub fn find_front_matter_in_mdast<TFrontMatter: DeserializeOwned>(
    mdast: &Node,
) -> Result<Option<TFrontMatter>, MdxError> {
    mdast
        .children()
        .into_iter()
        .flatten()
        .find_map(|child| match child {
            Node::Toml(Toml { value, .. }) => Some(value),
            _ => None,
        })
        .map(|front_matter| toml::from_str(front_matter).map_err(MdxError::ParseFrontMatter))
        .transpose()
}
