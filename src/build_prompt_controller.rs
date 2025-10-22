use anyhow::Result;
use anyhow::anyhow;

use crate::build_prompt_controller_params::BuildPromptControllerParams;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::prompt_controller::PromptController;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::string_to_mdast::string_to_mdast;

pub fn build_prompt_controller(
    BuildPromptControllerParams {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        file,
        rhai_template_renderer,
    }: BuildPromptControllerParams,
) -> Result<PromptController> {
    let mdast = string_to_mdast(&file.contents)?;
    let front_matter: PromptDocumentFrontMatter = find_front_matter_in_mdast(&mdast)?
        .ok_or_else(|| anyhow!("No front matter found in file: {:?}", file.relative_path))?;

    Ok(PromptController {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        front_matter,
        mdast,
        rhai_template_renderer,
    })
}
