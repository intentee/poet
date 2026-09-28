use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_html_flow_element_with_children() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r#"<div className="box">
x
</div>"#,
            &SyntaxSet::new()
        )??,
        r#"<div className="box" ><p>x</p></div>"#
    );

    Ok(())
}
