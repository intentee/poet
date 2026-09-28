use std::collections::BTreeMap;

use poet_mdx::string_to_mdast::string_to_mdast;
use poet_prompt::prompt_document_evaluator::PromptDocumentEvaluator;
use poet_prompt::prompt_document_front_matter::PromptDocumentFrontMatter;
use poet_prompt::prompt_error::PromptError;

use crate::fixture_rendering_context::fixture_rendering_context;
use crate::poet_prompt_tests_error::PoetPromptTestsError;

pub fn evaluate_prompt_markdown(
    markdown: &str,
) -> Result<Result<String, PromptError>, PoetPromptTestsError> {
    let rendering_context = fixture_rendering_context()?;
    let component_context = rendering_context.component_context(
        BTreeMap::new(),
        PromptDocumentFrontMatter {
            arguments: BTreeMap::new(),
            description: "Fixture description".to_owned(),
            title: "Fixture title".to_owned(),
        },
    );

    Ok(PromptDocumentEvaluator {
        component_context: &component_context,
        rhai_template_renderer: &rendering_context.rhai_template_renderer,
    }
    .eval(&string_to_mdast(markdown)?))
}
