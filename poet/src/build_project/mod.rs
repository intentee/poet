pub mod build_authors;
pub mod build_content;
pub mod build_project_params;
pub mod build_project_result;
pub mod build_project_result_holder;
pub mod build_project_result_stub;

use std::sync::Arc;

use anyhow::Result;
pub use build_authors::build_authors;
pub use build_content::build_content;
pub use build_content::build_content_params;
pub use build_project_params::BuildProjectParams;
pub use build_project_result::BuildProjectResult;
pub use build_project_result_holder::BuildProjectResultHolder;
pub use build_project_result_stub::BuildProjectResultStub;

use self::build_content::build_content_params::BuildContentParams;
use crate::filesystem::memory::Memory;

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
