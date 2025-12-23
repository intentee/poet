use anyhow::Result;
use anyhow::anyhow;

use crate::build_prompt_document_controller_params::BuildPromptDocumentControllerParams;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::prompt_document_controller::PromptDocumentController;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::string_to_mdast::string_to_mdast;

pub fn build_prompt_document_controller(
    BuildPromptDocumentControllerParams {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        file,
        name,
        rhai_template_renderer,
    }: BuildPromptDocumentControllerParams,
) -> Result<PromptDocumentController> {
    let mdast = string_to_mdast(&file.contents)?;
    let front_matter: PromptDocumentFrontMatter = find_front_matter_in_mdast(&mdast)?
        .ok_or_else(|| anyhow!("No front matter found in file: {:?}", file.relative_path))?;

    Ok(PromptDocumentController {
        asset_path_renderer,
        content_document_linker,
        esbuild_metafile,
        front_matter,
        name,
        mdast,
        rhai_template_renderer,
    })
}
