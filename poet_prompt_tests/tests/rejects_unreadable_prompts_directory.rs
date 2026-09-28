use std::fs::write;

use poet_content_tests::fixture_project::FixtureProject;
use poet_filesystem::storage::Storage;
use poet_prompt::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use poet_prompt::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::fixture_rendering_context::fixture_rendering_context;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[tokio::test]
async fn rejects_unreadable_prompts_directory() -> Result<(), PoetPromptTestsError> {
    let fixture_project = FixtureProject::create()?;
    let regular_file = fixture_project.directory.path().join("regular-file");

    write(&regular_file, "not a directory")?;

    assert!(matches!(
        build_prompt_document_controller_collection(
            BuildPromptDocumentControllerCollectionParams {
                rendering_context: fixture_rendering_context()?,
                source_filesystem: &Storage {
                    base_directory: regular_file,
                },
            }
        )
        .await,
        Err(PromptError::ReadPromptFiles(_))
    ));

    Ok(())
}
