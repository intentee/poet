use std::str::FromStr as _;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::author_collection::AuthorCollection;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_prompt::prompt_rendering_context::PromptRenderingContext;
use poet_prompt::register_prompt_rhai_types::register_prompt_rhai_types;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::create_component_engine::create_component_engine;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

use crate::poet_prompt_tests_error::PoetPromptTestsError;

pub fn fixture_rendering_context() -> Result<PromptRenderingContext, PoetPromptTestsError> {
    let mut expression_engine = create_component_engine();

    register_prompt_rhai_types(&mut expression_engine);

    Ok(PromptRenderingContext {
        asset_path_renderer: AssetPathRenderer {
            base_path: "/".to_owned(),
        },
        content_document_linker: fixture_site_context(
            &[fixture_reference(
                "guide",
                "description = \"Guide\"\nlayout = \"Layout\"\ntitle = \"Guide\"",
            )?],
            AuthorCollection::default(),
        )?
        .content_document_linker,
        esbuild_metafile: Arc::new(EsbuildMetafile::from_str(include_str!(
            "../fixtures/metafile.json"
        ))?),
        rhai_template_renderer: RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: Arc::new(ComponentRegistry::default()),
            expression_engine,
        })?,
    })
}
