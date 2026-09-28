use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_paragraph_with_emphasis_and_strong() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"*one* **two**", &SyntaxSet::new())??,
        r"<p><em>one</em> <strong>two</strong></p>"
    );

    Ok(())
}
