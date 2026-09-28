use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn resolves_internal_link_through_linker() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"[label](guide)", &SyntaxSet::new())??,
        r#"<p><a href="/guide/">label</a></p>"#
    );

    Ok(())
}
