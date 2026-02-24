use crate::blog_name::BlogName;
use crate::blog_post_basename::BlogPostBasename;
use crate::blog_post_front_matter::BlogPostFrontMatter;

#[derive(Clone, Debug)]
pub struct BlogPost {
    pub basename: BlogPostBasename,
    pub blog_name: BlogName,
    pub front_matter: BlogPostFrontMatter,
}
