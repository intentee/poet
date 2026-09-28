use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::generated_file::generated_file;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn renders_mdx_component_and_expression() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutMinimal.rhai",
            include_str!("../fixtures/LayoutMinimal.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "shortcodes/PrimaryNavigation.rhai",
            include_str!("../fixtures/PrimaryNavigation.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "content/index.md",
            "+++\ndescription = \"Home\"\nlayout = \"LayoutMinimal\"\ntitle = \"Home\"\n+++\n\nValue {40 + 2}\n\n<PrimaryNavigation>\ninner\n</PrimaryNavigation>\n",
        )
        .await?;

    let home = generated_file(&fixture_project.build(false).await?, "index.html").await?;

    assert!(home.contains("<p>Value 42</p>"));
    assert!(home.contains("<nav><p>inner</p></nav>"));

    Ok(())
}
