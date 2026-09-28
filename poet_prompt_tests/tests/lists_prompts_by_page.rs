use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_argument::PromptArgument;
use poet_prompt_tests::build_fixture_prompts::build_fixture_prompts;
use poet_prompt_tests::fixture_prompt_file::FixturePromptFile;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

const PLAIN_PROMPT: &str =
    "+++\narguments = {}\ndescription = \"Plain\"\ntitle = \"Plain\"\n+++\n\n**user**: hi\n";

#[tokio::test]
async fn lists_prompts_by_page() -> Result<(), PoetPromptTestsError> {
    let prompt_document_controller_collection = build_fixture_prompts(&[
        FixturePromptFile {
            contents: PLAIN_PROMPT,
            relative_path: "prompts/alpha.md",
        },
        FixturePromptFile {
            contents: "+++\ndescription = \"Reviews code\"\ntitle = \"Review\"\n\n[arguments.language]\ndescription = \"Programming language\"\nrequired = false\ntitle = \"Language\"\n\n[arguments.code]\ndescription = \"Code to review\"\nrequired = true\ntitle = \"Code\"\n+++\n\n**user**: review\n",
            relative_path: "prompts/beta.md",
        },
        FixturePromptFile {
            contents: PLAIN_PROMPT,
            relative_path: "prompts/gamma.md",
        },
    ])
    .await??;

    let listed_prompts: Vec<Prompt> =
        prompt_document_controller_collection.list_mcp_prompts(ListResourcesCursor {
            offset: 1,
            per_page: 1,
        });

    assert!(matches!(
        listed_prompts.as_slice(),
        [Prompt {
            arguments,
            description,
            name,
            title,
        }] if description == "Reviews code"
            && name == "beta"
            && title == "Review"
            && matches!(
                arguments.as_slice(),
                [
                    PromptArgument { name: first_name, required: true, .. },
                    PromptArgument { name: second_name, required: false, .. },
                ] if first_name == "code" && second_name == "language"
            )
    ));

    Ok(())
}
