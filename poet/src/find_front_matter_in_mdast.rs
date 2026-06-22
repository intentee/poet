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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde::Deserialize;

    use super::*;
    use crate::string_to_mdast::string_to_mdast;

    #[derive(Deserialize)]
    struct TestFrontMatter {
        title: String,
    }

    #[test]
    fn extracts_toml_front_matter_from_document() -> Result<()> {
        let mdast = string_to_mdast("+++\ntitle = \"Hello\"\n+++\n\nBody text")?;
        let front_matter: Option<TestFrontMatter> = find_front_matter_in_mdast(&mdast)?;

        assert_eq!(
            front_matter.map(|front_matter| front_matter.title),
            Some("Hello".to_string())
        );

        Ok(())
    }

    #[test]
    fn returns_none_when_document_has_no_front_matter() -> Result<()> {
        let mdast = string_to_mdast("Just body text")?;
        let front_matter: Option<TestFrontMatter> = find_front_matter_in_mdast(&mdast)?;

        assert!(front_matter.is_none());

        Ok(())
    }
}
