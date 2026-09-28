use poet_mdx::mdx_error::MdxError;
use poet_mdx::string_to_mdast::string_to_mdast;

#[test]
fn rejects_invalid_markdown() {
    assert!(matches!(
        string_to_mdast("{"),
        Err(MdxError::ParseMarkdown { .. })
    ));
}
