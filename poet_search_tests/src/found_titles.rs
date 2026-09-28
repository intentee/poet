use poet_search::search_error::SearchError;
use poet_search::search_index_found_document::SearchIndexFoundDocument;
use poet_search::search_index_query_params::SearchIndexQueryParams;
use poet_search::search_index_reader::SearchIndexReader;

pub fn found_titles(
    search_index_reader: &SearchIndexReader,
    query: &str,
) -> Result<Vec<String>, SearchError> {
    Ok(search_index_reader
        .query(SearchIndexQueryParams {
            offset: 0,
            per_page: search_index_reader.content_document_sources.len(),
            query: query.to_owned(),
        })?
        .into_iter()
        .map(
            |SearchIndexFoundDocument {
                 content_document_reference,
             }| content_document_reference.front_matter.title.clone(),
        )
        .collect())
}
