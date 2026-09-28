use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn rejects_table_of_contents_in_heading_while_collecting_it() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(r"## {context.table_of_contents}", &SyntaxSet::new())?,
        Err(ContentError::EvaluateExpression { .. })
    ));

    Ok(())
}
