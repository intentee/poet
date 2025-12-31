use std::sync::Arc;

use crate::author_collection::AuthorCollection;
use crate::filesystem::storage::Storage;

pub struct BuildBlogsParams {
    pub authors: AuthorCollection,
    pub source_filesystem: Arc<Storage>,
}
