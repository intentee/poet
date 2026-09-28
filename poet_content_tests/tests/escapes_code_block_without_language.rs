use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn escapes_code_block_without_language() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"```
a < b
```",
            &SyntaxSet::new()
        )??,
        r#"<pre class="code"><code>a &lt; b</code></pre>"#
    );

    Ok(())
}
