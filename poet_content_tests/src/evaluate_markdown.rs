use poet_content::author_collection::AuthorCollection;
use poet_content::content_document_evaluator::ContentDocumentEvaluator;
use poet_content::content_error::ContentError;
use poet_mdx::string_to_mdast::string_to_mdast;
use syntect::parsing::SyntaxSet;

use crate::fixture_component_context::fixture_component_context;
use crate::fixture_reference::fixture_reference;
use crate::fixture_site_context::fixture_site_context;
use crate::fixture_template_renderer::fixture_template_renderer;
use crate::poet_content_tests_error::PoetContentTestsError;

pub fn evaluate_markdown(
    markdown: &str,
    syntax_set: &SyntaxSet,
) -> Result<Result<String, ContentError>, PoetContentTestsError> {
    let guide_reference = fixture_reference(
        "guide",
        "description = \"Guide\"\nid = \"guide-id\"\nlayout = \"Layout\"\ntitle = \"Guide\"",
    )?;
    let hidden_reference = fixture_reference(
        "hidden",
        "description = \"Hidden\"\nlayout = \"Layout\"\nrender = false\ntitle = \"Hidden\"",
    )?;
    let component_context = fixture_component_context(
        vec![],
        guide_reference.clone(),
        fixture_site_context(
            &[guide_reference, hidden_reference],
            AuthorCollection::default(),
        )?,
    )?;

    Ok(ContentDocumentEvaluator {
        component_context: &component_context,
        rhai_template_renderer: &fixture_template_renderer()?,
        syntax_set,
    }
    .eval(&string_to_mdast(markdown)?))
}
