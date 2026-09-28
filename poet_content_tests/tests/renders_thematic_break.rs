use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_thematic_break() -> Result<(), PoetContentTestsError> {
    assert_eq!(evaluate_markdown(r"***", &SyntaxSet::new())??, r"<hr>");

    Ok(())
}
