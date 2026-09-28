use poet_mdx::find_text_content_in_mdast::find_text_content_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;

#[test]
fn finds_text_in_every_container_kind() -> Result<(), PoetMdxTestsError> {
    assert_eq!(
        find_text_content_in_mdast(&string_to_mdast(
            "> quoted ~~struck~~ [linked](https://example.com) <Foo.Bar>jsx</Foo.Bar>\n\n| head |\n| ---- |\n| cell |\n"
        )?),
        "quoted struck linked jsxheadcell"
    );

    Ok(())
}
