use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;
use crate::search_index_reader::SearchIndexReader;

#[derive(Clone, Default)]
pub struct SearchIndexReaderHolder {
    search_index_reader: Arc<RwLock<Option<Arc<SearchIndexReader>>>>,
    pub update_notifier: Arc<Notify>,
}

impl Holder for SearchIndexReaderHolder {
    type Item = Arc<SearchIndexReader>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.search_index_reader.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
