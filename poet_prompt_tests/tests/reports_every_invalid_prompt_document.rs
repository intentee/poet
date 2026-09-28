use poet_mdx::document_error::DocumentError;
use poet_mdx::mdx_error::MdxError;
use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::build_fixture_prompts::build_fixture_prompts;
use poet_prompt_tests::fixture_prompt_file::FixturePromptFile;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[tokio::test]
async fn reports_every_invalid_prompt_document() -> Result<(), PoetPromptTestsError> {
    let Err(PromptError::InvalidPromptDocuments(document_errors)) = build_fixture_prompts(&[
        FixturePromptFile {
            contents: "{\n",
            relative_path: "prompts/broken.md",
        },
        FixturePromptFile {
            contents: "**user**: hi\n",
            relative_path: "prompts/missing.md",
        },
        FixturePromptFile {
            contents: "+++\ntitle = \"Missing required fields\"\n+++\n\n**user**: hi\n",
            relative_path: "prompts/unknown.md",
        },
    ])
    .await?
    else {
        panic!("expected invalid prompt documents");
    };

    assert!(matches!(
        document_errors.into_document_errors().as_slice(),
        [
            DocumentError {
                basename: broken_basename,
                error: PromptError::ParsePromptDocument(MdxError::ParseMarkdown { .. }),
            },
            DocumentError {
                basename: missing_basename,
                error: PromptError::MissingFrontMatter,
            },
            DocumentError {
                basename: unknown_basename,
                error: PromptError::ParsePromptDocument(MdxError::ParseFrontMatter(_)),
            },
        ] if broken_basename == "broken" && missing_basename == "missing" && unknown_basename == "unknown"
    ));

    Ok(())
}
