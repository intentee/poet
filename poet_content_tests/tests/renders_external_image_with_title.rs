use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_external_image_with_title() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r#"![photo](https://example.com/p.png "Caption")"#,
            &SyntaxSet::new()
        )??,
        r#"<p><img alt="photo" src="https://example.com/p.png" title="Caption"></p>"#
    );

    Ok(())
}
