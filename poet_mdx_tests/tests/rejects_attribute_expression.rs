use markdown::mdast::AttributeContent;
use markdown::mdast::MdxJsxExpressionAttribute;
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn rejects_attribute_expression() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[AttributeContent::Expression(MdxJsxExpressionAttribute {
            stops: vec![],
            value: "...props".to_owned(),
        })],
        &[],
        &FixtureComponentContext,
        String::new(),
        Some(&"div".to_owned()),
        &fixture_renderer().await?,
    );

    assert!(matches!(
        evaluated_element,
        Err(MdxError::AttributeExpressionNotSupported { tag_name }) if tag_name == "div"
    ));

    Ok(())
}
