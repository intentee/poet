use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use poet_content::build_project_result::BuildProjectResult;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

pub struct FilesystemHttpRouteIndexBuilder {
    pub build_project_result_holder: Holder<BuildProjectResult>,
    pub ctrlc_notifier: CancellationToken,
    pub filesystem_http_route_index_holder: Holder<Arc<FilesystemHttpRouteIndex>>,
}

impl FilesystemHttpRouteIndexBuilder {
    fn build_filesystem_http_route_index(&self) {
        match self.build_project_result_holder.get() {
            HolderState::Ready(BuildProjectResult {
                generated_files, ..
            }) => self.filesystem_http_route_index_holder.set(Arc::new(
                FilesystemHttpRouteIndex::from_generated_files(&generated_files),
            )),
            HolderState::NotReady => debug!("Build project results not ready yet. Skipping build"),
        }
    }
}

#[async_trait]
impl Service for FilesystemHttpRouteIndexBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        let mut build_project_result_updates = self.build_project_result_holder.subscribe();

        loop {
            self.build_filesystem_http_route_index();

            tokio::select! {
                Ok(()) = build_project_result_updates.changed() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
