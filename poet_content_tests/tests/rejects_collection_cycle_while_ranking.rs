use poet_content::content_error::ContentError;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn rejects_collection_cycle_while_ranking() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "content/first.md",
            "+++\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"t\"\n\n[[collection]]\nafter = \"second\"\nname = \"loop\"\n+++\n",
        )
        .await?;
    fixture_project
        .add_file(
            "content/second.md",
            "+++\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"t\"\n\n[[collection]]\nafter = \"first\"\nname = \"loop\"\n+++\n",
        )
        .await?;

    assert!(matches!(
        fixture_project.build(false).await,
        Err(PoetContentTestsError::Content(
            ContentError::CollectionCycle { .. }
        ))
    ));

    Ok(())
}
