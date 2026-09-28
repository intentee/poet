use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn escapes_html_in_inline_code() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"`<div>`", &SyntaxSet::new())??,
        r"<p><code>&lt;div&gt;</code></p>"
    );

    Ok(())
}
