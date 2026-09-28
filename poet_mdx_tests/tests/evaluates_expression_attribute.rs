use markdown::mdast::AttributeContent;
use markdown::mdast::AttributeValue;
use markdown::mdast::AttributeValueExpression;
use markdown::mdast::MdxJsxAttribute;
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn evaluates_expression_attribute() -> Result<(), PoetMdxTestsError> {
    let evaluated_element = eval_mdx_element(
        &[AttributeContent::Property(MdxJsxAttribute {
            name: "data-label".to_owned(),
            value: Some(AttributeValue::Expression(AttributeValueExpression {
                stops: vec![],
                value: "\"hi\" + \"!\"".to_owned(),
            })),
        })],
        &[],
        &FixtureComponentContext,
        String::new(),
        Some(&"div".to_owned()),
        &fixture_renderer().await?,
    );

    assert_eq!(evaluated_element?, r#"<div data-label="hi!" ></div>"#);

    Ok(())
}
