use poet_content::build_project_result::BuildProjectResult;
use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

async fn project_with_guide(body: &str) -> Result<FixtureProject, PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "content/guide.md",
            &format!(
                "+++\ndescription = \"d\"\nlayout = \"LayoutPlain\"\ntitle = \"Guide\"\n+++\n\n{body}\n"
            ),
        )
        .await?;

    Ok(fixture_project)
}

#[tokio::test]
async fn reports_document_with_changed_content() -> Result<(), PoetContentTestsError> {
    let previous_build =
        BuildProjectResult::from(project_with_guide("body").await?.build(false).await?);
    let current_build = project_with_guide("changed body")
        .await?
        .build(false)
        .await?
        .changed_compared_to(&previous_build);

    assert_eq!(current_build.changed_since_last_build.len(), 1);

    Ok(())
}
