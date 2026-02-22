use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use tokio_util::sync::CancellationToken;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::service::Service;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::filesystem_http_route_index_holder::FilesystemHttpRouteIndexHolder;
use crate::holder::Holder as _;

pub struct FilesystemHttpRouteIndexBuilder {
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub filesystem_http_route_index_holder: FilesystemHttpRouteIndexHolder,
}

impl FilesystemHttpRouteIndexBuilder {
    async fn do_build_filesystem_http_route_index(&self) {
        let BuildProjectResult {
            memory_filesystem, ..
        } = match self.build_project_result_holder.get().await {
            Some(build_project_result) => build_project_result,
            None => {
                debug!("Build project results not ready yet. Skipping build");

                return;
            }
        };

        self.filesystem_http_route_index_holder
            .set(Some(Arc::new(
                match FilesystemHttpRouteIndex::from_filesystem(memory_filesystem).await {
                    Ok(filesystem_http_route_index) => filesystem_http_route_index,
                    Err(err) => {
                        error!("Unable to build filesysetm http route index: {err:#?}");

                        return;
                    }
                },
            )))
            .await;
    }
}

#[async_trait]
impl Service for FilesystemHttpRouteIndexBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_filesystem_http_route_index().await;

            tokio::select! {
                _ = self.build_project_result_holder.update_notifier.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
