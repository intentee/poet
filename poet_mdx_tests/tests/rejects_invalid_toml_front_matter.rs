use poet_mdx::find_front_matter_in_mdast::find_front_matter_in_mdast;
use poet_mdx::mdx_error::MdxError;
use poet_mdx::string_to_mdast::string_to_mdast;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use serde::de::IgnoredAny;

#[test]
fn rejects_invalid_toml_front_matter() -> Result<(), PoetMdxTestsError> {
    assert!(matches!(
        find_front_matter_in_mdast::<IgnoredAny>(&string_to_mdast("+++\ntitle = \n+++\n")?),
        Err(MdxError::ParseFrontMatter(_))
    ));

    Ok(())
}
