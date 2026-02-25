use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;
use crate::semantic_search_index::SemanticSearchIndex;

#[derive(Clone, Default)]
pub struct SemanticSearchIndexHolder {
    semantic_search_index: Arc<RwLock<Option<Arc<SemanticSearchIndex>>>>,
    pub update_notifier: Arc<Notify>,
}

impl Holder for SemanticSearchIndexHolder {
    type Item = Arc<SemanticSearchIndex>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.semantic_search_index.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
