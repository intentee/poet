use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_footnote_reference() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"note[^a]

[^a]: detail",
            &SyntaxSet::new()
        )??,
        r##"<p>note<a href="#footnote-a" role="doc-noteref">a</a></p>"##
    );

    Ok(())
}
