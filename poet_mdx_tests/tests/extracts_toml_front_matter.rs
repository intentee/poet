use poet_mdx::find_front_matter_in_mdast::find_front_matter_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use serde::Deserialize;

#[derive(Deserialize)]
struct FixtureFrontMatter {
    title: String,
}

#[test]
fn extracts_toml_front_matter() -> Result<(), PoetMdxTestsError> {
    let front_matter: Option<FixtureFrontMatter> = find_front_matter_in_mdast(&string_to_mdast(
        "+++\ntitle = \"Hello\"\n+++\n\nBody text",
    )?)?;

    assert_eq!(
        front_matter.map(|FixtureFrontMatter { title }| title),
        Some("Hello".to_owned())
    );

    Ok(())
}
