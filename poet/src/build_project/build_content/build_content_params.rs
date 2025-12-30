use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::author_collection::AuthorCollection;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;

pub struct BuildContentParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub authors: AuthorCollection,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub generated_page_base_path: String,
    pub is_watching: bool,
    pub memory_filesystem: Arc<Memory>,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: Arc<Storage>,
}
