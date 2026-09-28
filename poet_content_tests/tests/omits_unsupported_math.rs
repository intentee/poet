use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn omits_unsupported_math() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"$$
x = 1
$$",
            &SyntaxSet::new()
        )??,
        r""
    );

    Ok(())
}
