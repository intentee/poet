use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use poet_content::build_project_result::BuildProjectResult;
use poet_search::search_index::SearchIndex;
use poet_search::search_index_reader::SearchIndexReader;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;
use crate::report_poet_error::report_poet_error;

pub struct SearchIndexBuilder {
    pub build_project_result_holder: Holder<BuildProjectResult>,
    pub ctrlc_notifier: CancellationToken,
    pub search_index_reader_holder: Holder<Arc<SearchIndexReader>>,
}

impl SearchIndexBuilder {
    fn build_search_index(&self) {
        let HolderState::Ready(BuildProjectResult {
            content_document_sources,
            ..
        }) = self.build_project_result_holder.get()
        else {
            debug!("Build project results not ready yet. Skipping build");

            return;
        };

        SearchIndex::create_in_memory(content_document_sources)
            .index()
            .map(|search_index_reader| {
                self.search_index_reader_holder
                    .set(Arc::new(search_index_reader));
            })
            .map_err(PoetError::IndexSearch)
            .unwrap_or_else(report_poet_error);
    }
}

#[async_trait]
impl Service for SearchIndexBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        let mut build_project_result_updates = self.build_project_result_holder.subscribe();

        loop {
            self.build_search_index();

            tokio::select! {
                Ok(()) = build_project_result_updates.changed() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
