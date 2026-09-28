use poet_content::author_collection::AuthorCollection;
use poet_content::content_document_evaluator::ContentDocumentEvaluator;
use poet_content::content_error::ContentError;
use poet_content_tests::fixture_component_context::fixture_component_context;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::fixture_template_renderer::fixture_template_renderer;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::string_to_mdast::string_to_mdast;
use syntect::parsing::SyntaxSet;

#[test]
fn rejects_table_of_contents_with_failing_heading() -> Result<(), PoetContentTestsError> {
    let reference = fixture_reference(
        "guide",
        "description = \"d\"\nlayout = \"L\"\ntitle = \"Guide\"",
    )?;
    let component_context = fixture_component_context(
        vec![],
        reference.clone(),
        fixture_site_context(&[reference], AuthorCollection::default())?,
    )?;

    assert!(matches!(
        ContentDocumentEvaluator {
            component_context: &component_context,
            rhai_template_renderer: &fixture_template_renderer()?,
            syntax_set: &SyntaxSet::new(),
        }
        .table_of_contents(&string_to_mdast("Intro\n\n## [label](ghost)")?),
        Err(ContentError::LinkedDocumentNotFound { .. })
    ));

    Ok(())
}
