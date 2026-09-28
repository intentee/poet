use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::document_error::DocumentError;

#[tokio::test]
async fn reports_failing_heading_while_collecting_table_of_contents()
-> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file("content/index.md", "+++\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"t\"\n+++\n\n## [label](ghost)\n")
        .await?;

    let Err(PoetContentTestsError::Content(ContentError::InvalidDocuments(document_errors))) =
        fixture_project.build(false).await
    else {
        panic!("expected invalid documents");
    };

    assert!(matches!(
        document_errors.into_document_errors().as_slice(),
        [DocumentError {
            error: ContentError::LinkedDocumentNotFound { .. },
            ..
        }]
    ));

    Ok(())
}
