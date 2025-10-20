use std::sync::Arc;

use crate::filesystem::storage::Storage;

pub struct BuildPromptControllerCollectionParams {
    pub source_filesystem: Arc<Storage>,
}
