use poet_content::content_error::ContentError;
use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_link_to_missing_document() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        evaluate_prompt_markdown("[x](ghost)")?,
        Err(PromptError::ResolveLink(ContentError::LinkedDocumentNotFound { basename })) if basename.0 == "ghost"
    ));

    Ok(())
}
