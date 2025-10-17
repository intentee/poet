use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use anyhow::Result;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::front_matter::FrontMatter;

#[derive(Clone, Debug)]
pub struct ContentDocumentReference {
    pub basename_path: PathBuf,
    pub front_matter: FrontMatter,
    pub generated_page_base_path: String,
}

impl ContentDocumentReference {
    pub fn basename(&self) -> String {
        self.basename_path.display().to_string()
    }

    pub fn canonical_link(&self) -> Result<String, String> {
        Ok(format!(
            "{}{}",
            self.generated_page_base_path,
            self.basename_link_stem()?
        )
        .to_string())
    }

    /// Starts without leading slash
    pub fn target_file_relative_path(&self) -> Result<PathBuf, String> {
        Ok(format!("{}index.html", self.basename_link_stem()?).into())
    }

    fn basename_link_stem(&self) -> Result<String, String> {
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
                    return Err(format!(
                        "Unable to get parent path for {}",
                        self.basename_path.display()
                    ));
                }
            };
            let file_stem = match self.basename_path.file_stem() {
                Some(file_stem) => file_stem.display().to_string(),
                None => {
                    return Err(format!(
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

    fn rhai_canonical_link(&mut self) -> Result<String, Box<EvalAltResult>> {
        Ok(self.canonical_link()?)
    }

    fn rhai_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }
}

impl CustomType for ContentDocumentReference {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentReference")
            .with_get("basename", Self::rhai_basename)
            .with_get("canonical_link", Self::rhai_canonical_link)
            .with_get("front_matter", Self::rhai_front_matter);
    }
}

impl Eq for ContentDocumentReference {}

impl Hash for ContentDocumentReference {
    fn hash<THasher: Hasher>(&self, state: &mut THasher) {
        self.basename_path.hash(state);
    }
}

impl Ord for ContentDocumentReference {
    fn cmp(&self, other: &Self) -> Ordering {
        self.basename_path.cmp(&other.basename_path)
    }
}

impl PartialEq for ContentDocumentReference {
    fn eq(&self, other: &Self) -> bool {
        self.basename_path == other.basename_path
    }
}

impl PartialOrd for ContentDocumentReference {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn target_path_is_generated_for_base_index() -> Result<()> {
        let reference = ContentDocumentReference {
            basename_path: "index".into(),
            front_matter: FrontMatter::mock("foo"),
            generated_page_base_path: "/".to_string(),
        };

        assert_eq!(reference.canonical_link().unwrap(), "/");

        assert_eq!(
            reference
                .target_file_relative_path()
                .unwrap()
                .display()
                .to_string(),
            "index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated_for_base() -> Result<()> {
        let reference = ContentDocumentReference {
            basename_path: "bar".into(),
            front_matter: FrontMatter::mock("foo"),
            generated_page_base_path: "/".to_string(),
        };

        assert_eq!(reference.canonical_link().unwrap(), "/bar/");

        assert_eq!(
            reference
                .target_file_relative_path()
                .unwrap()
                .display()
                .to_string(),
            "bar/index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated() -> Result<()> {
        let reference = ContentDocumentReference {
            basename_path: "foo/bar".into(),
            front_matter: FrontMatter::mock("foo"),
            generated_page_base_path: "/".to_string(),
        };

        assert_eq!(reference.canonical_link().unwrap(), "/foo/bar/");

        assert_eq!(
            reference
                .target_file_relative_path()
                .unwrap()
                .display()
                .to_string(),
            "foo/bar/index.html"
        );

        Ok(())
    }

    #[test]
    fn target_path_is_generated_for_index() -> Result<()> {
        let reference = ContentDocumentReference {
            basename_path: "foo/index".into(),
            front_matter: FrontMatter::mock("foo"),
            generated_page_base_path: "/".to_string(),
        };

        assert_eq!(reference.canonical_link().unwrap(), "/foo/");

        assert_eq!(
            reference
                .target_file_relative_path()
                .unwrap()
                .display()
                .to_string(),
            "foo/index.html"
        );

        Ok(())
    }
}
