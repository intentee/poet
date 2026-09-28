use poet_content_tests::build_fixture_guide::build_fixture_guide;
use poet_search::search_index::SearchIndex;
use poet_search::search_index_reader::SearchIndexReader;

use crate::poet_search_tests_error::PoetSearchTestsError;

pub async fn index_fixture_document(
    description: &str,
    body: &str,
) -> Result<SearchIndexReader, PoetSearchTestsError> {
    Ok(SearchIndex::create_in_memory(
        build_fixture_guide(description, body)
            .await?
            .content_document_sources,
    )
    .index()?)
}
