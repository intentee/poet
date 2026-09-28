use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_code_block_verbatim() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("```rust\nlet path = \"src/main.rs\";\nif a < b {}\n```")??,
        "```rust\nlet path = \"src/main.rs\";\nif a < b {}\n```"
    );

    Ok(())
}
