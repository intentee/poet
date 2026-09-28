use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn escapes_quotes_in_link_href() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r#"[label](https://example.com/?q="x")"#, &SyntaxSet::new())??,
        r#"<p><a href="https://example.com/?q=&quot;x&quot;">label</a></p>"#
    );

    Ok(())
}
