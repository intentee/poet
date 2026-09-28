use markdown::mdast::AttributeContent;
use markdown::mdast::AttributeValue;
use markdown::mdast::MdxJsxAttribute;
use markdown::mdast::Node;
use markdown::mdast::Text;
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx_tests::fixture_component_context::FixtureComponentContext;
use poet_mdx_tests::fixture_renderer::fixture_renderer;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[tokio::test]
async fn renders_html_element_with_literal_and_boolean_attributes() -> Result<(), PoetMdxTestsError>
{
    let evaluated_element = eval_mdx_element(
        &[
            AttributeContent::Property(MdxJsxAttribute {
                name: "class".to_owned(),
                value: Some(AttributeValue::Literal("highlight".to_owned())),
            }),
            AttributeContent::Property(MdxJsxAttribute {
                name: "hidden".to_owned(),
                value: None,
            }),
        ],
        &[Node::Text(Text {
            position: None,
            value: "child".to_owned(),
        })],
        &FixtureComponentContext,
        "child".to_owned(),
        Some(&"div".to_owned()),
        &fixture_renderer().await?,
    );

    assert_eq!(
        evaluated_element?,
        r#"<div class="highlight" hidden >child</div>"#
    );

    Ok(())
}
