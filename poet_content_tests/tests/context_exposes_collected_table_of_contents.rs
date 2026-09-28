use poet_content::table_of_contents::TableOfContents;
use poet_content::table_of_contents_heading::TableOfContentsHeading;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_exposes_collected_table_of_contents() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("docs/second")?.with_table_of_contents(
                TableOfContents {
                    headings: vec![TableOfContentsHeading {
                        content: "Intro".to_owned(),
                        depth: 2,
                        id: "intro".to_owned(),
                    }],
                }
            ),
            r#"let heading = context.table_of_contents.headings[0]; heading.id + ":" + heading.content + ":" + heading.depth"#,
        )?,
        "intro:Intro:2"
    );

    Ok(())
}
