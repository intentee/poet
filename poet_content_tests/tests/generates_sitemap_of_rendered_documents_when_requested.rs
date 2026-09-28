use poet_content_tests::fixture_project::FixtureProject;
use poet_content_tests::generated_file_contents::generated_file_contents;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[tokio::test]
async fn generates_sitemap_of_rendered_documents_when_requested()
-> Result<(), PoetContentTestsError> {
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
            "+++\ndescription = \"Home\"\nlayout = \"LayoutPlain\"\ntitle = \"Home\"\n+++\n\nHome.\n",
        )
        .await?;
    fixture_project
        .add_file(
            "content/draft.md",
            "+++\ndescription = \"Draft\"\nlayout = \"LayoutPlain\"\nrender = false\ntitle = \"Draft\"\n+++\n\nDraft.\n",
        )
        .await?;

    let build_project_result_stub = fixture_project.build(true).await?;
    let sitemap = generated_file_contents(&build_project_result_stub, "sitemap.xml")?;

    assert!(sitemap.contains("<loc>/</loc>"));
    assert!(!sitemap.contains("<loc>/draft/</loc>"));
    assert_eq!(build_project_result_stub.content_document_sources.len(), 1);

    Ok(())
}
