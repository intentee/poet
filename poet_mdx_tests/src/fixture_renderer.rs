use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::poet_mdx_tests_error::PoetMdxTestsError;
use crate::register_fixture_rhai_types::register_fixture_rhai_types;
use crate::shortcodes_storage::ShortcodesStorage;

pub async fn fixture_renderer() -> Result<RhaiTemplateRenderer, PoetMdxTestsError> {
    let ShortcodesStorage {
        directory: _shortcodes_directory,
        storage,
    } = ShortcodesStorage::with_shortcode(
        "Emphasized",
        "fn template(context, props, content) { component { <em>{content}</em> } }",
    )
    .await?;

    Ok(compile_shortcodes(CompileShortcodesParams {
        register_rhai_types: register_fixture_rhai_types,
        source_filesystem: &storage,
    })
    .await?)
}
