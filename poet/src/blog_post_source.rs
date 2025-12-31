use markdown::mdast::Node;

use crate::blog_post_reference::BlogPostReference;
use crate::filesystem::file_entry::FileEntry;

#[derive(Clone)]
pub struct BlogPostSource {
    pub file_entry: FileEntry,
    pub mdast: Node,
    pub reference: BlogPostReference,
    pub relative_path: String,
}
