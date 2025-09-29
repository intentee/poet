use std::sync::Arc;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::holder::Holder;

#[derive(Clone, Default)]
pub struct BuildProjectResultHolder {
    build_project_result_lock: Arc<RwLock<Option<BuildProjectResult>>>,
    pub total: Arc<AtomicUsize>,
    pub update_notifier: Arc<Notify>,
}

impl BuildProjectResultHolder {
    pub async fn must_get_build_project_result(&self) -> Result<BuildProjectResult> {
        self.get().await.ok_or_else(|| {
            anyhow!("Server is still starting up, or there are no successful builds yet")
        })
    }
}

#[async_trait]
impl Holder for BuildProjectResultHolder {
    type Item = BuildProjectResult;

    fn on_update(&self, build_project_result: &Option<Self::Item>) {
        self.total.store(
            if let Some(build_project_result) = build_project_result {
                build_project_result
                    .markdown_document_reference_collection
                    .len()
            } else {
                0
            },
            atomic::Ordering::Relaxed,
        );
    }

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.build_project_result_lock.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
