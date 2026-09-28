#[must_use]
pub fn markdown_code_fence(code: &str, minimum_length: usize) -> String {
    let longest_backtick_run = code
        .split(|character| character != '`')
        .map(str::len)
        .fold(0, usize::max);

    "`".repeat(minimum_length.max(longest_backtick_run + 1))
}
