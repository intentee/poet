use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::generated_file::generated_file;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn rich_layout_uses_every_template_accessor() -> Result<(), PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file(
            "shortcodes/LayoutRich.rhai",
            include_str!("../fixtures/LayoutRich.rhai"),
        )
        .await?;
    fixture_project
        .add_file(
            "shortcodes/LayoutPlain.rhai",
            include_str!("../fixtures/LayoutPlain.rhai"),
        )
        .await?;
    fixture_project
        .add_file("authors/alice.toml", "name = \"Alice\"")
        .await?;
    fixture_project
        .add_file(
            "content/index.md",
            "+++\ndescription = \"Welcome home\"\nlayout = \"LayoutRich\"\ntitle = \"Home Page\"\nauthors = [\"alice\"]\n+++\n\n# Section One\n\nHome body.\n",
        )
        .await?;
    fixture_project
        .add_file(
            "content/docs/index.md",
            "+++\ndescription = \"Docs\"\nlayout = \"LayoutPlain\"\ntitle = \"Docs\"\n\n[[collection]]\nname = \"docs\"\n+++\n\nDocs index.\n",
        )
        .await?;
    fixture_project
        .add_file(
            "content/docs/page.md",
            "+++\ndescription = \"Page\"\nlayout = \"LayoutPlain\"\ntitle = \"Page\"\n\n[[collection]]\nname = \"docs\"\nafter = \"docs/index\"\n+++\n\nPage body.\n",
        )
        .await?;

    let home = generated_file(&fixture_project.build(false).await?, "index.html").await?;

    assert!(home.contains("<title>Home Page</title>"));
    assert!(home.contains("<p>Welcome home</p>"));
    assert!(home.contains("<i>is-rendered</i>"));
    assert!(home.contains("reftitle:Home Page"));
    assert!(home.contains("watching:false"));
    assert!(home.contains("authors:1"));
    assert!(home.contains("<b>alice=Alice</b>"));
    assert!(home.contains("<li>section-one:Section One:1</li>"));
    assert!(home.contains("<h2>docs</h2>"));
    assert!(home.contains("<li>docs/index in docs kids 0</li><li>docs/page in docs kids 0</li>"));

    Ok(())
}
