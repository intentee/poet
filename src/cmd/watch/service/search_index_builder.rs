use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use tokio_util::sync::CancellationToken;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::watch::service::Service;
use crate::holder::Holder as _;
use crate::search_index::SearchIndex;

pub struct SearchIndexBuilder {
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
}

#[async_trait]
impl Service for SearchIndexBuilder {
    async fn run(self: Arc<Self>) -> Result<()> {
        let do_build_search_index = async || {
            let BuildProjectResult {
                esbuild_metafile: _,
                markdown_document_sources,
                memory_filesystem: _,
            } = match self.build_project_result_holder.get().await {
                Some(build_project_result) => build_project_result,
                None => {
                    debug!("Rhai components are not compiled yet. Skipping build");

                    return;
                }
            };

            let search_index = SearchIndex::create_in_ram();

            if let Err(err) =
                search_index.index_markdown_document_sources(markdown_document_sources)
            {
                error!("Unable to index markdown document sources: {err:#?}");

                return;
            }

            let search_index_reader = match search_index.reader() {
                Ok(search_index_reader) => search_index_reader,
                Err(err) => {
                    error!("Unable to create search index reader: {err:#?}");

                    return;
                }
            };
        };

        loop {
            tokio::select! {
                _ = self.build_project_result_holder.update_notifier.notified() => do_build_search_index().await,
                _ = self.ctrlc_notifier.cancelled() => {
                    break;
                },
            }
        }

        Ok(())
    }
}
