use poet_filesystem::storage::Storage;
use poet_mdx::compile_shortcodes::compile_shortcodes;
use poet_mdx::compile_shortcodes_params::CompileShortcodesParams;
use poet_mdx::mdx_error::MdxError;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::register_poet_rhai_types::register_poet_rhai_types;

pub async fn compile_poet_shortcodes(
    source_filesystem: &Storage,
) -> Result<RhaiTemplateRenderer, MdxError> {
    compile_shortcodes(CompileShortcodesParams {
        register_rhai_types: register_poet_rhai_types,
        source_filesystem,
    })
    .await
}
