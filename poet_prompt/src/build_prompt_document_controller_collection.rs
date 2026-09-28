use std::sync::Arc;

use dashmap::DashMap;
use log::info;
use poet_filesystem::filesystem::Filesystem;
use poet_mdx::build_timer::BuildTimer;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;

use crate::build_prompt_document_controller::build_prompt_document_controller;
use crate::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use crate::build_prompt_document_controller_params::BuildPromptDocumentControllerParams;
use crate::prompt_document_controller::PromptDocumentController;
use crate::prompt_document_controller_collection::PromptDocumentControllerCollection;
use crate::prompt_error::PromptError;
use crate::prompts_source_directory::PROMPTS_SOURCE_DIRECTORY;

pub async fn build_prompt_document_controller_collection<TFilesystem: Filesystem>(
    BuildPromptDocumentControllerCollectionParams {
        rendering_context,
        source_filesystem,
    }: BuildPromptDocumentControllerCollectionParams<'_, TFilesystem>,
) -> Result<PromptDocumentControllerCollection, PromptError> {
    info!("Processing prompt files...");

    let _build_timer = BuildTimer::default();
    let document_errors: DocumentErrorCollection<PromptError> = DocumentErrorCollection::default();
    let prompt_document_controllers: DashMap<String, PromptDocumentController> = DashMap::new();
    let rendering_context = Arc::new(rendering_context);

    source_filesystem
        .read_source_files(&PROMPTS_SOURCE_DIRECTORY)
        .await
        .map_err(PromptError::ReadPromptFiles)?
        .into_par_iter()
        .for_each(|source_file| {
            let document_label = source_file.stem_path.display().to_string();

            match build_prompt_document_controller(BuildPromptDocumentControllerParams {
                rendering_context: rendering_context.clone(),
                source_file,
            }) {
                Ok(prompt_document_controller) => {
                    prompt_document_controllers.insert(
                        prompt_document_controller.name.clone(),
                        prompt_document_controller,
                    );
                }
                Err(prompt_error) => document_errors.register_error(document_label, prompt_error),
            }
        });

    if document_errors.is_empty() {
        Ok(PromptDocumentControllerCollection {
            prompt_document_controllers: prompt_document_controllers.into_iter().collect(),
        })
    } else {
        Err(PromptError::InvalidPromptDocuments(document_errors))
    }
}
