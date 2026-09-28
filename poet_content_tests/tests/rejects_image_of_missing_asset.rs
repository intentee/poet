use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn rejects_image_of_missing_asset() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(r"![logo](missing.png)", &SyntaxSet::new())?,
        Err(ContentError::ResolveImage(_))
    ));

    Ok(())
}
