use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn rejects_link_to_missing_document() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(r"[label](ghost)", &SyntaxSet::new())?,
        Err(ContentError::LinkedDocumentNotFound { .. })
    ));

    Ok(())
}
