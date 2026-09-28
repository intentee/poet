use poet_mdx::find_text_content_in_mdast::find_text_content_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[test]
fn concatenates_text_across_nested_containers() -> Result<(), PoetMdxTestsError> {
    assert_eq!(
        find_text_content_in_mdast(&string_to_mdast(
            "# Hello **bold** and *italic*\n\n- item `code`"
        )?),
        "Hello bold and italicitem "
    );

    Ok(())
}
