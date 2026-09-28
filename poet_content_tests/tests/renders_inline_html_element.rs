use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_inline_html_element() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"a <x.y>b</x.y> c", &SyntaxSet::new())??,
        r"<p>a <x.y >b</x.y> c</p>"
    );

    Ok(())
}
