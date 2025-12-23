use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::esbuild_metafile_holder::EsbuildMetaFileHolder;
use crate::filesystem::storage::Storage;
use crate::holder::Holder as _;
use crate::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;

pub struct EsbuildMetaFileReader {
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: EsbuildMetaFileHolder,
    pub on_esbuild_metafile_changed: Arc<Notify>,
    pub source_filesystem: Arc<Storage>,
}

#[async_trait]
impl Service for EsbuildMetaFileReader {
    async fn run(&self) -> Result<()> {
        loop {
            match read_esbuild_metafile_or_default(self.source_filesystem.clone()).await {
                Ok(esbuild_metafile) => {
                    self.esbuild_metafile_holder
                        .set(Some(esbuild_metafile))
                        .await;
                }
                Err(err) => {
                    self.esbuild_metafile_holder.set(None).await;

                    error!("Unable to read esbuild metafile: {err:#?}");
                }
            }

            tokio::select! {
                _ = self.on_esbuild_metafile_changed.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
