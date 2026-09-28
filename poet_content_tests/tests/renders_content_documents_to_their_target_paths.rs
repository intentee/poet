use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::generated_file::generated_file;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn renders_content_documents_to_their_target_paths() -> Result<(), PoetContentTestsError> {
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
            "+++\ndescription = \"Home\"\nlayout = \"LayoutPlain\"\ntitle = \"Home\"\n+++\n\nHome body.\n",
        )
        .await?;
    fixture_project
        .add_file(
            "content/docs/page.md",
            "+++\ndescription = \"Page\"\nlayout = \"LayoutPlain\"\ntitle = \"Page\"\n+++\n\nPage body.\n",
        )
        .await?;

    let build_project_result_stub = fixture_project.build(false).await?;

    assert_eq!(
        generated_file(&build_project_result_stub, "index.html").await?,
        "\n    <html><p>Home body.</p></html>\n  "
    );
    assert_eq!(
        generated_file(&build_project_result_stub, "docs/page/index.html").await?,
        "\n    <html><p>Page body.</p></html>\n  "
    );

    Ok(())
}
