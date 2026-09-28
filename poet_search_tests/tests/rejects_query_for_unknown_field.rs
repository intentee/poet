use poet_search::search_error::SearchError;
use poet_search::search_index_query_params::SearchIndexQueryParams;
use poet_search_tests::index_fixture_document::index_fixture_document;
use poet_search_tests::poet_search_tests_error::PoetSearchTestsError;
use tantivy::query::QueryParserError;

#[tokio::test]
async fn rejects_query_for_unknown_field() -> Result<(), PoetSearchTestsError> {
    let search_index_reader = index_fixture_document("Guide", "Body.").await?;

    assert!(matches!(
        search_index_reader.query(SearchIndexQueryParams {
            offset: 0,
            per_page: 1,
            query: "author:zebra".to_owned(),
        }),
        Err(SearchError::ParseQuery {
            source: QueryParserError::FieldDoesNotExist(field_name),
            ..
        }) if field_name == "author"
    ));

    Ok(())
}
