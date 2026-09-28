use std::sync::Arc;

use poet::holder::Holder;
use poet::mcp_prompt_provider_prompt_documents::McpPromptProviderPromptDocuments;
use poet_mcp::provider_error::ProviderError;
use poet_prompt_tests::build_fixture_prompts::build_fixture_prompts;
use poet_prompt_tests::fixture_prompt_file::FixturePromptFile;

pub async fn prompt_provider_with_fixture_prompt()
-> Result<McpPromptProviderPromptDocuments, ProviderError> {
    let prompt_document_controller_collection_holder = Holder::default();

    prompt_document_controller_collection_holder.set(Arc::new(
        build_fixture_prompts(&[FixturePromptFile {
            contents: "+++\narguments = {}\ndescription = \"Greets\"\ntitle = \"Greet\"\n+++\n\n**user**: Hello!\n",
            relative_path: "prompts/greet.md",
        }])
        .await??,
    ));

    Ok(McpPromptProviderPromptDocuments {
        prompt_document_controller_collection_holder,
    })
}
