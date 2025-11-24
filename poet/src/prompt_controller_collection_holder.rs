use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;
use crate::mcp::prompt_controller_collection::PromptControllerCollection;

#[derive(Clone, Default)]
pub struct PromptControllerCollectionHolder {
    prompt_controller_collection: Arc<RwLock<Option<Arc<PromptControllerCollection>>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for PromptControllerCollectionHolder {
    type Item = Arc<PromptControllerCollection>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.prompt_controller_collection.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
