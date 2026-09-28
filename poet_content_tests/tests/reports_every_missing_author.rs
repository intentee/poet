use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::document_error::DocumentError;

#[tokio::test]
async fn reports_every_missing_author() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "content/index.md",
            "+++\nauthors = [\"ghost\", \"phantom\"]\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"t\"\n+++\n",
        )
        .await?;

    let Err(PoetContentTestsError::Content(ContentError::InvalidDocuments(document_errors))) =
        fixture_project.build(false).await
    else {
        panic!("expected invalid documents");
    };

    assert_eq!(
        document_errors
            .into_document_errors()
            .into_iter()
            .filter_map(|DocumentError { error, .. }| match error {
                ContentError::AuthorNotFound { author_name, .. } => Some(author_name),
                _ => None,
            })
            .collect::<Vec<String>>(),
        vec!["ghost".to_owned(), "phantom".to_owned()]
    );

    Ok(())
}
