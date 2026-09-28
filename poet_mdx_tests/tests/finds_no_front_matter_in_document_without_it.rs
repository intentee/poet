use poet_mdx::find_front_matter_in_mdast::find_front_matter_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use serde::de::IgnoredAny;

#[test]
fn finds_no_front_matter_in_document_without_it() -> Result<(), PoetMdxTestsError> {
    assert!(
        find_front_matter_in_mdast::<IgnoredAny>(&string_to_mdast("Just body text")?)?.is_none()
    );

    Ok(())
}
