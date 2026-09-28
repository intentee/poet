use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use poet_mdx::mdx_error::MdxError;
use poet_mdx_tests::poet_mdx_tests_error::PoetMdxTestsError;
use poet_mdx_tests::register_fixture_rhai_types::register_fixture_rhai_types;
use poet_mdx_tests::shortcodes_storage::ShortcodesStorage;

#[tokio::test]
async fn rejects_shortcode_with_invalid_syntax() -> Result<(), PoetMdxTestsError> {
    let ShortcodesStorage {
        directory: _shortcodes_directory,
        storage,
    } = ShortcodesStorage::with_shortcode("Broken", "fn template( { @ not rhai @ }").await?;

    assert!(matches!(
        compile_shortcodes(CompileShortcodesParams {
            register_rhai_types: register_fixture_rhai_types,
            source_filesystem: &storage,
        })
        .await,
        Err(MdxError::BuildTemplateRenderer(_))
    ));

    Ok(())
}
