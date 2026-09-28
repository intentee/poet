use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn rejects_content_file_with_invalid_front_matter() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file("content/index.md", "+++\ntitle = \n+++\n\nBody.\n")
        .await?;

    assert!(matches!(
        fixture_project.build(false).await,
        Err(PoetContentTestsError::Content(
            ContentError::ParseContentDocument { .. }
        ))
    ));

    Ok(())
}
