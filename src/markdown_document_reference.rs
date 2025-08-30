use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use rhai::CustomType;
use rhai::TypeBuilder;

use crate::front_matter::FrontMatter;

#[derive(Clone, Debug)]
pub struct MarkdownDocumentReference {
    pub basename_path: PathBuf,
    pub front_matter: FrontMatter,
}

impl MarkdownDocumentReference {
    pub fn basename(&self) -> String {
        self.basename_path.display().to_string()
    }

    /// Starts with leading slash
    pub fn canonical_link(&self) -> Result<String> {
        Ok(format!("/{}", self.basename_link_stem()?).to_string())
    }

    /// Starts without leading slash
    pub fn target_file_relative_path(&self) -> Result<PathBuf> {
        Ok(format!("{}index.html", self.basename_link_stem()?).into())
    }

    fn basename_link_stem(&self) -> Result<String> {
        if self.basename_path.ends_with("index") {
            if let Some(parent) = self.basename_path.parent() {
                let parent_str = parent.display().to_string();

                if parent_str.is_empty() {
                    Ok("".into())
                } else {
                    Ok(format!("{}/", parent_str))
                }
            } else {
                Ok("".into())
            }
        } else {
            let parent = match self.basename_path.parent() {
                Some(parent) => parent.display().to_string(),
                None => {
                    return Err(anyhow!(
                        "Unable to get parent path for {}",
                        self.basename_path.display()
                    ));
                }
            };
            let file_stem = match self.basename_path.file_stem() {
                Some(file_stem) => file_stem.display().to_string(),
                None => {
                    return Err(anyhow!(
                        "Unable to get file stem path for {}",
                        self.basename_path.display()
                    ));
                }
            };

            if parent.is_empty() {
                Ok(format!("{file_stem}/"))
            } else {
                Ok(format!("{parent}/{file_stem}/"))
            }
        }
    }

    fn rhai_basename(&mut self) -> String {
        self.basename()
    }

    fn rhai_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }
}

impl CustomType for MarkdownDocumentReference {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("MarkdownDocumentReference")
            .with_get("basename", Self::rhai_basename)
            .with_get("front_matter", Self::rhai_front_matter);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn target_path_is_generated_for_base_index() -> Result<()> {
        let reference = MarkdownDocumentReference {
            basename_path: "index".into(),
            front_matter: FrontMatter::mock("foo"),
        };

        assert_eq!(reference.canonical_link()?, "/");

        assert_eq!(
            reference.target_file_relative_path()?.display().to_string(),
            "index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated_for_base() -> Result<()> {
        let reference = MarkdownDocumentReference {
            basename_path: "bar".into(),
            front_matter: FrontMatter::mock("foo"),
        };

        assert_eq!(reference.canonical_link()?, "/bar/");

        assert_eq!(
            reference.target_file_relative_path()?.display().to_string(),
            "bar/index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated() -> Result<()> {
        let reference = MarkdownDocumentReference {
            basename_path: "foo/bar".into(),
            front_matter: FrontMatter::mock("foo"),
        };

        assert_eq!(reference.canonical_link()?, "/foo/bar/");

        assert_eq!(
            reference.target_file_relative_path()?.display().to_string(),
            "foo/bar/index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated_for_index() -> Result<()> {
        let reference = MarkdownDocumentReference {
            basename_path: "foo/index".into(),
            front_matter: FrontMatter::mock("foo"),
        };

        assert_eq!(reference.canonical_link()?, "/foo/");

        assert_eq!(
            reference.target_file_relative_path()?.display().to_string(),
            "foo/index.html"
        );

        Ok(())
    }
}
