use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_unordered_list() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"- first
- second",
            &SyntaxSet::new()
        )??,
        r"<ul><li><p>first</p></li><li><p>second</p></li></ul>"
    );

    Ok(())
}
