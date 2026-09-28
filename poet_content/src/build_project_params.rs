use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::author_collection::AuthorCollection;

pub struct BuildProjectParams<'params, TFilesystem> {
    pub asset_path_renderer: AssetPathRenderer,
    pub authors: AuthorCollection,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub generate_sitemap: bool,
    pub generated_page_base_path: String,
    pub is_watching: bool,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: &'params TFilesystem,
}
