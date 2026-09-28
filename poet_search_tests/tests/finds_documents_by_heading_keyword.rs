use poet_search_tests::found_titles::found_titles;
use poet_search_tests::index_fixture_document::index_fixture_document;
use poet_search_tests::poet_search_tests_error::PoetSearchTestsError;

#[tokio::test]
async fn finds_documents_by_heading_keyword() -> Result<(), PoetSearchTestsError> {
    let search_index_reader = index_fixture_document("Guide", "## Caring for lemurs").await?;

    assert_eq!(
        found_titles(&search_index_reader, "lemurs")?,
        vec!["Guide".to_owned()]
    );

    Ok(())
}
