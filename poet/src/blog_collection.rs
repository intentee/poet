use std::collections::BTreeMap;

use crate::blog::Blog;
use crate::blog_name::BlogName;

#[derive(Clone, Debug, Default)]
pub struct BlogCollection {
    blogs: BTreeMap<BlogName, Blog>,
}

impl BlogCollection {
    pub fn insert(&mut self, blog: Blog) {
        self.blogs.insert(blog.name.clone(), blog);
    }
}
