pub mod build_prompt_controller_collection_params;
pub mod build_prompt_controller_collection_result;

use anyhow::Result;
use log::info;

use crate::build_prompt_controller::build_prompt_controller;
use crate::build_prompt_controller_collection::build_prompt_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::build_prompt_controller_collection::build_prompt_controller_collection_result::BuildPromptControllerCollectionResult;
use crate::build_prompt_controller_params::BuildPromptControllerParams;
use crate::build_timer::BuildTimer;
use crate::filesystem::Filesystem as _;

pub async fn build_prompt_controller_collection(
    BuildPromptControllerCollectionParams {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        rhai_template_renderer,
        source_filesystem,
    }: BuildPromptControllerCollectionParams,
) -> Result<BuildPromptControllerCollectionResult> {
    info!("Processing prompt files...");

    let _build_timer = BuildTimer::new();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_prompt() {
            build_prompt_controller(BuildPromptControllerParams {
                asset_path_renderer: asset_path_renderer.clone(),
                content_document_linker: content_document_linker.clone(),
                esbuild_metafile: esbuild_metafile.clone(),
                file,
                rhai_template_renderer: rhai_template_renderer.clone(),
            })
            .await?;
        }
    }

    Ok(BuildPromptControllerCollectionResult {})
}
