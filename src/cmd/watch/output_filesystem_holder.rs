use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::filesystem::Filesystem;

pub struct OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    pub output_filesystem: RwLock<Option<Arc<TFilesystem>>>,
    pub update_notifier: Notify,
}

impl<TFilesystem> OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    pub async fn get_output_filesystem(&self) -> Result<Option<Arc<TFilesystem>>> {
        let output_filesystem = self.output_filesystem.read().await;

        Ok(output_filesystem.clone())
    }

    pub async fn set_output_filesystem(&self, filesystem: Arc<TFilesystem>) -> Result<()> {
        {
            let mut output_filesystem = self.output_filesystem.write().await;

            *output_filesystem = Some(filesystem);
        }

        self.update_notifier.notify_waiters();

        Ok(())
    }
}

impl<TFilesystem> Default for OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    fn default() -> Self {
        Self {
            output_filesystem: RwLock::new(None),
            update_notifier: Notify::new(),
        }
    }
}
