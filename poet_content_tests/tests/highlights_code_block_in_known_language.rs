use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::fixture_syntax_set::fixture_syntax_set;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn highlights_code_block_in_known_language() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(
            r"```demo
fn main
```",
            &fixture_syntax_set(include_str!("../fixtures/demo.sublime-syntax"))?
        )??,
        r#"<pre class="code language-demo" data-lang="demo"><code><span class="source demo"><span class="keyword demo">fn</span> main</span></code></pre>"#
    );

    Ok(())
}
