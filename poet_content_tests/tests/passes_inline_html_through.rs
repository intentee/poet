use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn passes_inline_html_through() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"a <span>b</span> c", &SyntaxSet::new())??,
        r"<p>a <span>b</span> c</p>"
    );

    Ok(())
}
