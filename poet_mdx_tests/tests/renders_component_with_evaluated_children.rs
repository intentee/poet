use markdown::mdast::Node;
use markdown::mdast::Text;
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn renders_component_with_evaluated_children() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[],
        &[Node::Text(Text {
            position: None,
            value: "child".to_owned(),
        })],
        &FixtureComponentContext,
        "child".to_owned(),
        Some(&"Emphasized".to_owned()),
        &fixture_renderer().await?,
    );

    assert_eq!(evaluated_element?, " <em>child</em> ");

    Ok(())
}
