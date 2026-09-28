use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_filesystem::storage::Storage;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::author_collection::AuthorCollection;

pub struct BuildProjectParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub authors: AuthorCollection,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub generated_page_base_path: String,
    pub generate_sitemap: bool,
    pub is_watching: bool,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: Arc<Storage>,
}
