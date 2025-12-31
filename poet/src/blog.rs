use markdown::mdast::Node;

use crate::blog_post_reference::BlogPostReference;

pub struct BlogPost {
    pub mdast: Node,
    pub reference: BlogPostReference,
}
