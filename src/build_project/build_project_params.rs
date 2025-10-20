use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::filesystem::storage::Storage;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub struct BuildProjectParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub generated_page_base_path: String,
    pub is_watching: bool,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: Arc<Storage>,
}
