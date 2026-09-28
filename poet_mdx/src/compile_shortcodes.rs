use std::sync::Arc;

use log::info;
use poet_filesystem::filesystem::Filesystem as _;
use rhai::module_resolvers::FileModuleResolver;
use rhai_components::component_syntax::component_reference::ComponentReference;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::create_component_engine::create_component_engine;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

use crate::build_timer::BuildTimer;
use crate::compile_shortcodes_params::CompileShortcodesParams;
use crate::mdx_error::MdxError;
use crate::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;

pub async fn compile_shortcodes(
    CompileShortcodesParams {
        register_rhai_types,
        source_filesystem,
    }: CompileShortcodesParams<'_>,
) -> Result<RhaiTemplateRenderer, MdxError> {
    info!("Compiling shortcodes...");

    let _build_timer = BuildTimer::default();
    let component_registry: Arc<ComponentRegistry> = Arc::default();

    for shortcode_file in source_filesystem
        .read_source_files(&SHORTCODES_SOURCE_DIRECTORY)
        .await
        .map_err(MdxError::ReadShortcodes)?
    {
        component_registry.register_component(ComponentReference {
            name: shortcode_file
                .stem_in(&SHORTCODES_SOURCE_DIRECTORY)
                .map_err(MdxError::ResolveShortcodeName)?,
        });
    }

    let mut expression_engine = create_component_engine();

    expression_engine.set_module_resolver(FileModuleResolver::new_with_path(
        source_filesystem
            .base_directory
            .join(SHORTCODES_SOURCE_DIRECTORY.name),
    ));
    register_rhai_types(&mut expression_engine);

    RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
        component_registry,
        expression_engine,
    })
    .map_err(MdxError::BuildTemplateRenderer)
}
