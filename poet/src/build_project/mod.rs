pub mod build_authors;
pub mod build_content;
pub mod build_project_result;
pub mod build_project_result_holder;
pub mod build_project_result_stub;

use std::sync::Arc;

use anyhow::Result;
pub use build_authors::build_authors;
pub use build_content::build_content;
pub use build_content::build_content_params;
pub use build_project_result::BuildProjectResult;
pub use build_project_result_holder::BuildProjectResultHolder;
pub use build_project_result_stub::BuildProjectResultStub;
use esbuild_metafile::EsbuildMetaFile;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use self::build_content::build_content_params::BuildContentParams;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;

pub struct BuildProjectParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub generated_page_base_path: String,
    pub is_watching: bool,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: Arc<Storage>,
}

pub async fn build_project(
    BuildProjectParams {
        asset_path_renderer,
        esbuild_metafile,
        generated_page_base_path,
        is_watching,
        rhai_template_renderer,
        source_filesystem,
    }: BuildProjectParams,
) -> Result<BuildProjectResultStub> {
    let authors = build_authors(source_filesystem.clone()).await?;
    let memory_filesystem = Arc::new(Memory::default());

    build_content(BuildContentParams {
        asset_path_renderer,
        authors,
        esbuild_metafile,
        generated_page_base_path,
        is_watching,
        memory_filesystem,
        rhai_template_renderer,
        source_filesystem,
    })
    .await
}
