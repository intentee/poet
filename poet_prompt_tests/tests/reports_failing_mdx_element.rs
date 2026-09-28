use poet_mdx::mdx_error::MdxError;
use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn reports_failing_mdx_element() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        evaluate_prompt_markdown("a <Missing.Component /> b")?,
        Err(PromptError::EvaluateMdxElement(
            MdxError::RenderComponent { .. }
        ))
    ));

    Ok(())
}
