use poet_content::author_collection::AuthorCollection;
use poet_content::content_document_evaluator::ContentDocumentEvaluator;
use poet_content::table_of_contents_heading::TableOfContentsHeading;
use poet_content_tests::fixture_component_context::fixture_component_context;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::fixture_template_renderer::fixture_template_renderer;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::string_to_mdast::string_to_mdast;
use syntect::parsing::SyntaxSet;

#[test]
fn collects_table_of_contents_headings() -> Result<(), PoetContentTestsError> {
    let reference = fixture_reference(
        "guide",
        "description = \"d\"\nlayout = \"L\"\ntitle = \"Guide\"",
    )?;
    let component_context = fixture_component_context(
        vec![],
        reference.clone(),
        fixture_site_context(&[reference], AuthorCollection::default())?,
    )?;
    let table_of_contents = ContentDocumentEvaluator {
        component_context: &component_context,
        rhai_template_renderer: &fixture_template_renderer()?,
        syntax_set: &SyntaxSet::new(),
    }
    .table_of_contents(&string_to_mdast(
        "# First *Part*\n\nText\n\n> ## Quoted\n\n- ### Listed",
    )?)?;

    assert_eq!(
        table_of_contents
            .headings
            .iter()
            .map(|TableOfContentsHeading { content, depth, id }| format!("{id}:{content}:{depth}"))
            .collect::<Vec<String>>(),
        vec![
            "first-part:First <em>Part</em>:1".to_owned(),
            "quoted:Quoted:2".to_owned(),
            "listed:Listed:3".to_owned(),
        ]
    );

    Ok(())
}
