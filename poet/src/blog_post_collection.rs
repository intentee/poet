use std::collections::BTreeMap;

use crate::blog_post::BlogPost;
use crate::blog_post_basename::BlogPostBasename;

#[derive(Clone, Debug, Default)]
pub struct BlogPostCollection {
    posts: BTreeMap<BlogPostBasename, BlogPost>,
}

impl BlogPostCollection {
    pub fn insert(&mut self, post: BlogPost) {
        self.posts.insert(post.basename.clone(), post);
    }
}
