use std::path::PathBuf;
use std::sync::Arc;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::file_entry_stub::FileEntryStub;
use poet_filesystem::source_file::SourceFile;
use poet_mcp::jsonrpc_version::JSONRPC_VERSION;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_request_params::PromptsGetRequestParams;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::request_id::RequestId;
use poet_prompt::build_prompt_document_controller::build_prompt_document_controller;
use poet_prompt::build_prompt_document_controller_params::BuildPromptDocumentControllerParams;
use poet_prompt::prompt_error::PromptError;

use crate::fixture_argument_input::FixtureArgumentInput;
use crate::fixture_rendering_context::fixture_rendering_context;
use crate::poet_prompt_tests_error::PoetPromptTestsError;

pub fn respond_to_prompt_document(
    contents: &str,
    argument_inputs: &[FixtureArgumentInput<'_>],
) -> Result<Result<PromptsGetResult, PromptError>, PoetPromptTestsError> {
    Ok(
        build_prompt_document_controller(BuildPromptDocumentControllerParams {
            rendering_context: Arc::new(fixture_rendering_context()?),
            source_file: SourceFile {
                file_entry: FileEntry::from(FileEntryStub {
                    contents: contents.to_owned(),
                    relative_path: PathBuf::from("prompts/fixture.md"),
                }),
                stem_path: PathBuf::from("fixture"),
            },
        })?
        .respond_to(PromptsGetRequest {
            id: RequestId::Number(1),
            jsonrpc: JSONRPC_VERSION.to_owned(),
            params: PromptsGetRequestParams {
                arguments: argument_inputs
                    .iter()
                    .map(|FixtureArgumentInput { input, name }| {
                        ((*name).to_owned(), (*input).to_owned())
                    })
                    .collect(),
                meta: None,
                name: "fixture".to_owned(),
            },
        }),
    )
}
