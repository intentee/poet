use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn escapes_quotes_in_code_block_language() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r#"```a"b
code
```"#,
            &SyntaxSet::new()
        )??,
        r#"<pre class="code language-a&quot;b" data-lang="a&quot;b"><code>code</code></pre>"#
    );

    Ok(())
}
