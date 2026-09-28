use std::sync::Arc;

use async_trait::async_trait;
use poet_prompt::prompt_document_controller_collection::PromptDocumentControllerCollection;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;

#[derive(Clone, Default)]
pub struct PromptDocumentControllerCollectionHolder {
    prompt_document_controller_collection:
        Arc<RwLock<Option<Arc<PromptDocumentControllerCollection>>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for PromptDocumentControllerCollectionHolder {
    type Item = Arc<PromptDocumentControllerCollection>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.prompt_document_controller_collection.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
