use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::fixture_syntax_set::fixture_syntax_set;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn reports_failing_code_highlighting() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(r"```unresolved
x
```", &fixture_syntax_set(include_str!("../fixtures/unresolved.sublime-syntax"))?)?,
        Err(ContentError::HighlightCode { language, .. }) if language == "unresolved"
    ));

    Ok(())
}
