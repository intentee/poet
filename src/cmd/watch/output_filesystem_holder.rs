use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::filesystem::Filesystem;
use crate::holder::Holder;

pub struct OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    pub output_filesystem: Arc<RwLock<Option<Arc<TFilesystem>>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl<TFilesystem> Holder for OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    type Item = Arc<TFilesystem>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.output_filesystem.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}

impl<TFilesystem> Default for OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    fn default() -> Self {
        Self {
            output_filesystem: Default::default(),
            update_notifier: Default::default(),
        }
    }
}
