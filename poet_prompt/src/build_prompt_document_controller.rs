use poet_mdx::find_front_matter_in_mdast::find_front_matter_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;

use crate::build_prompt_document_controller_params::BuildPromptDocumentControllerParams;
use crate::prompt_document_controller::PromptDocumentController;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_error::PromptError;

pub fn build_prompt_document_controller(
    BuildPromptDocumentControllerParams {
        rendering_context,
        source_file,
    }: BuildPromptDocumentControllerParams,
) -> Result<PromptDocumentController, PromptError> {
    let name = source_file
        .stem_name()
        .map_err(PromptError::InvalidPromptName)?;
    let mdast = string_to_mdast(&source_file.file_entry.contents)
        .map_err(PromptError::ParsePromptDocument)?;
    let front_matter: PromptDocumentFrontMatter = find_front_matter_in_mdast(&mdast)
        .map_err(PromptError::ParsePromptDocument)?
        .ok_or(PromptError::MissingFrontMatter)?;

    Ok(PromptDocumentController {
        front_matter,
        mdast,
        name,
        rendering_context,
    })
}
