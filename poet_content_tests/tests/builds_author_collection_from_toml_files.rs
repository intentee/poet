use poet_content::author::Author;
use poet_content::author_data::AuthorData;
use poet_content::build_authors::build_authors;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn builds_author_collection_from_toml_files() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file("authors/team/alice.toml", "name = \"Alice\"")
        .await?;

    let resolved_authors = build_authors(&fixture_project.storage)
        .await?
        .resolve(&["team/alice".to_owned()]);

    assert!(matches!(
        resolved_authors.found_authors.as_slice(),
        [Author { data: AuthorData { name }, .. }] if name == "Alice"
    ));

    Ok(())
}
