use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn resolves_internal_image_through_asset_manager() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"![logo](logo.png)", &SyntaxSet::new())??,
        r#"<p><img alt="logo" src="/static/logo_ABCDEF12.png"></p>"#
    );

    Ok(())
}
