use std::path::PathBuf;

use crate::blog_post_basename::BlogPostBasename;
use crate::blog_post_front_matter::BlogPostFrontMatter;

#[derive(Clone, Debug)]
pub struct BlogPostReference {
    pub basename_path: PathBuf,
    pub front_matter: BlogPostFrontMatter,
}

impl BlogPostReference {
    pub fn basename(&self) -> BlogPostBasename {
        self.basename_path.clone().into()
    }
}
