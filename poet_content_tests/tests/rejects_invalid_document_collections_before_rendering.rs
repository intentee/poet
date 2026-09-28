use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn rejects_invalid_document_collections_before_rendering() -> Result<(), PoetContentTestsError>
{
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
            "+++\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"t\"\n\n[[collection]]\nafter = \"ghost\"\nname = \"docs\"\n+++\n\nBody.\n",
        )
        .await?;

    assert!(matches!(
        fixture_project.build(false).await,
        Err(PoetContentTestsError::Content(
            ContentError::InvalidDocuments(_)
        ))
    ));

    Ok(())
}
