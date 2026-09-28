use std::sync::Arc;

use async_trait::async_trait;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_filesystem::storage::Storage;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::poet_error::PoetError;
use crate::report_poet_error::report_poet_error;

pub struct EsbuildMetafileReader {
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: Holder<Arc<EsbuildMetafile>>,
    pub on_esbuild_metafile_changed: Arc<Notify>,
    pub source_filesystem: Arc<Storage>,
}

impl EsbuildMetafileReader {
    async fn read_esbuild_metafile(&self) {
        match read_esbuild_metafile_or_default(self.source_filesystem.as_ref()).await {
            Ok(esbuild_metafile) => self.esbuild_metafile_holder.set(esbuild_metafile),
            Err(asset_error) => {
                self.esbuild_metafile_holder.reset();

                report_poet_error(PoetError::ReadEsbuildMetafile(asset_error));
            }
        }
    }
}

#[async_trait]
impl Service for EsbuildMetafileReader {
    async fn run(&self) -> Result<(), PoetError> {
        loop {
            self.read_esbuild_metafile().await;

            tokio::select! {
                () = self.on_esbuild_metafile_changed.notified() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
