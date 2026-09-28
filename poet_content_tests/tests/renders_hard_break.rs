use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_hard_break() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"first\
second",
            &SyntaxSet::new()
        )??,
        r"<p>first<br>second</p>"
    );

    Ok(())
}
