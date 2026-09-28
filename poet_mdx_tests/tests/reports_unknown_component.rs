use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn reports_unknown_component() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[],
        &[],
        &FixtureComponentContext,
        String::new(),
        Some(&"Unknown".to_owned()),
        &fixture_renderer().await?,
    );

    assert!(matches!(
        evaluated_element,
        Err(MdxError::RenderComponent { tag_name, .. }) if tag_name == "Unknown"
    ));

    Ok(())
}
