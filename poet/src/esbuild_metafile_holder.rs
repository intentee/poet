use std::sync::Arc;

use async_trait::async_trait;
use esbuild_metafile::EsbuildMetaFile;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;

#[derive(Clone, Default)]
pub struct EsbuildMetaFileHolder {
    esbuild_metafile: Arc<RwLock<Option<Arc<EsbuildMetaFile>>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for EsbuildMetaFileHolder {
    type Item = Arc<EsbuildMetaFile>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.esbuild_metafile.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
