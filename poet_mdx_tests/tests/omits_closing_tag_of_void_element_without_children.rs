use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn omits_closing_tag_of_void_element_without_children() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[],
        &[],
        &FixtureComponentContext,
        String::new(),
        Some(&"br".to_owned()),
        &fixture_renderer().await?,
    );

    assert_eq!(evaluated_element?, "<br >");

    Ok(())
}
