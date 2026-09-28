use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_table() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"| H1 | H2 |
| -- | -- |
| a | b |",
            &SyntaxSet::new()
        )??,
        r"<table><tr><td>H1</td><td>H2</td></tr><tr><td>a</td><td>b</td></tr></table>"
    );

    Ok(())
}
