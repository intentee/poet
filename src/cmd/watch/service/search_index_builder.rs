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
use crate::search_index_reader::SearchIndexReader;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

pub struct SearchIndexBuilder {
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub search_index_reader_holder: SearchIndexReaderHolder,
}

impl SearchIndexBuilder {
    async fn do_build_search_index(&self) {
        let BuildProjectResult {
            markdown_document_sources,
            ..
        } = match self.build_project_result_holder.get().await {
            Some(build_project_result) => build_project_result,
            None => {
                debug!("Rhai components are not compiled yet. Skipping build");

                return;
            }
        };

        let search_index = SearchIndex::create_in_memory();

        if let Err(err) = search_index.index_markdown_document_sources(markdown_document_sources) {
            error!("Unable to index markdown document sources: {err:#?}");

            return;
        }

        let search_index_reader: SearchIndexReader = match search_index.try_into() {
            Ok(search_index_reader) => search_index_reader,
            Err(err) => {
                error!("Unable to create search index reader: {err:#?}");

                return;
            }
        };

        self.search_index_reader_holder
            .set(Some(Arc::new(search_index_reader))).await;
    }
}

#[async_trait]
impl Service for SearchIndexBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_search_index().await;

            tokio::select! {
                _ = self.build_project_result_holder.update_notifier.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
