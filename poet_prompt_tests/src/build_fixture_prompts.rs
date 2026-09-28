use poet_content_tests::fixture_project::FixtureProject;
use poet_prompt::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use poet_prompt::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use poet_prompt::prompt_document_controller_collection::PromptDocumentControllerCollection;
use poet_prompt::prompt_error::PromptError;

use crate::fixture_prompt_file::FixturePromptFile;
use crate::fixture_rendering_context::fixture_rendering_context;
use crate::poet_prompt_tests_error::PoetPromptTestsError;

pub async fn build_fixture_prompts(
    fixture_prompt_files: &[FixturePromptFile<'_>],
) -> Result<Result<PromptDocumentControllerCollection, PromptError>, PoetPromptTestsError> {
    let fixture_project = FixtureProject::create()?;

    for FixturePromptFile {
        contents,
        relative_path,
    } in fixture_prompt_files
    {
        fixture_project.add_file(relative_path, contents).await?;
    }

    Ok(
        build_prompt_document_controller_collection(
            BuildPromptDocumentControllerCollectionParams {
                rendering_context: fixture_rendering_context()?,
                source_filesystem: &fixture_project.storage,
            },
        )
        .await,
    )
}
