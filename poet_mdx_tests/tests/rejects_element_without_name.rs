use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn rejects_element_without_name() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[],
        &[],
        &FixtureComponentContext,
        String::new(),
        None,
        &fixture_renderer().await?,
    );

    assert!(matches!(
        evaluated_element,
        Err(MdxError::ElementWithoutName)
    ));

    Ok(())
}
