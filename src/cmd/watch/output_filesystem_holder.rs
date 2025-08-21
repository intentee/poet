use std::sync::RwLock;

use anyhow::Result;
use tokio::sync::watch;

use crate::filesystem::Filesystem;

pub struct OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    output_filesystem: RwLock<Option<TFilesystem>>,
    update_notifier: watch::Sender<()>,
}

impl<TFilesystem> OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    pub fn set_output_filesystem(&self, filesystem: TFilesystem) -> Result<()> {
        {
            let mut output_filesystem = self
                .output_filesystem
                .write()
                .expect("Failed to acquire write lock on output filesystem");
            *output_filesystem = Some(filesystem);
        }

        self.update_notifier.send(())?;

        Ok(())
    }
}

impl<TFilesystem> Default for OutputFilesystemHolder<TFilesystem>
where
    TFilesystem: Filesystem,
{
    fn default() -> Self {
        let (update_notifier, _) = watch::channel(());

        Self {
            output_filesystem: RwLock::new(None),
            update_notifier,
        }
    }
}
