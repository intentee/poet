use poet_content::content_error::ContentError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::expression_system_error_of::expression_system_error_of;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn context_rejects_link_to_missing_document() -> Result<(), PoetPromptTestsError> {
    let Err(prompt_error) = assemble_prompt_messages(r#"{context.link_to("ghost")}"#)? else {
        panic!("expected the link to fail");
    };

    assert!(matches!(
        expression_system_error_of::<ContentError>(&prompt_error),
        Some(ContentError::LinkedDocumentNotFound { basename }) if basename.0 == "ghost"
    ));

    Ok(())
}
