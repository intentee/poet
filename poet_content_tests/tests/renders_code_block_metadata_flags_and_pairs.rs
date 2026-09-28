use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_code_block_metadata_flags_and_pairs() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"```text highlighted label:foo
code
```",
            &SyntaxSet::new()
        )??,
        r#"<pre class="code language-text" data-lang="text" data-meta-line="highlighted label:foo" highlighted data-meta-label="foo"><code>code</code></pre>"#
    );

    Ok(())
}
