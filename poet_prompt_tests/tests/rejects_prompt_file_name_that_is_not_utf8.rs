use std::ffi::OsString;
use std::fs::create_dir;
use std::fs::write;
use std::os::unix::ffi::OsStringExt as _;

use poet_content_tests::fixture_project::FixtureProject;
use poet_mdx::document_error::DocumentError;
use poet_prompt::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use poet_prompt::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::fixture_rendering_context::fixture_rendering_context;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[tokio::test]
async fn rejects_prompt_file_name_that_is_not_utf8() -> Result<(), PoetPromptTestsError> {
    let fixture_project = FixtureProject::create()?;
    let prompts_directory = fixture_project.directory.path().join("prompts");

    create_dir(&prompts_directory)?;
    write(
        prompts_directory.join(OsString::from_vec(vec![0xff, b'.', b'm', b'd'])),
        "+++\narguments = {}\ndescription = \"d\"\ntitle = \"t\"\n+++\n",
    )?;

    let Err(PromptError::InvalidPromptDocuments(document_errors)) =
        build_prompt_document_controller_collection(
            BuildPromptDocumentControllerCollectionParams {
                rendering_context: fixture_rendering_context()?,
                source_filesystem: &fixture_project.storage,
            },
        )
        .await
    else {
        panic!("expected invalid prompt documents");
    };

    assert!(matches!(
        document_errors.into_document_errors().as_slice(),
        [DocumentError {
            error: PromptError::InvalidPromptName(_),
            ..
        }]
    ));

    Ok(())
}
