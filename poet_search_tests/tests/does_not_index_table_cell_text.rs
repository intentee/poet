use poet_search_tests::found_titles::found_titles;
use poet_search_tests::index_fixture_document::index_fixture_document;
use poet_search_tests::poet_search_tests_error::PoetSearchTestsError;

#[tokio::test]
async fn does_not_index_table_cell_text() -> Result<(), PoetSearchTestsError> {
    let search_index_reader =
        index_fixture_document("Guide", "| animal |\n| ------ |\n| narwhal |").await?;

    assert!(found_titles(&search_index_reader, "narwhal")?.is_empty());

    Ok(())
}
