use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_external_link_with_title() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r#"[label](https://example.com "Tooltip")"#,
            &SyntaxSet::new()
        )??,
        r#"<p><a href="https://example.com" title="Tooltip">label</a></p>"#
    );

    Ok(())
}
