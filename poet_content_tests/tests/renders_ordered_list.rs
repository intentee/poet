use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_ordered_list() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"1. first
2. second",
            &SyntaxSet::new()
        )??,
        r"<ol><li><p>first</p></li><li><p>second</p></li></ol>"
    );

    Ok(())
}
