use crate::blog_config::BlogConfig;
use crate::blog_name::BlogName;
use crate::blog_post_collection::BlogPostCollection;

#[derive(Clone, Debug)]
pub struct Blog {
    pub name: BlogName,
    pub config: BlogConfig,
    pub posts: BlogPostCollection,
}
