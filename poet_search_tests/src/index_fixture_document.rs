use poet_content_tests::fixture_project::FixtureProject;
use poet_search::search_index::SearchIndex;
use poet_search::search_index_reader::SearchIndexReader;

use crate::poet_search_tests_error::PoetSearchTestsError;

pub async fn index_fixture_document(
    description: &str,
    body: &str,
) -> Result<SearchIndexReader, PoetSearchTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/Layout.rhai",
            include_str!("../fixtures/Layout.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "content/guide.md",
            &format!(
                "+++\ndescription = \"{description}\"\nlayout = \"Layout\"\ntitle = \"Searchable Guide\"\n+++\n\n{body}\n"
            ),
        )
        .await?;

    Ok(
        SearchIndex::create_in_memory(fixture_project.build(false).await?.content_document_sources)
            .index()?,
    )
}
