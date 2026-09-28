use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_strikethrough() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"~~gone~~", &SyntaxSet::new())??,
        r"<p><del>gone</del></p>"
    );

    Ok(())
}
