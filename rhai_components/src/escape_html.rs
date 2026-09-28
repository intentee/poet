/// Taken from Tera: <https://github.com/Keats/tera/blob/master/src/utils.rs>
#[must_use]
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for char in input.chars() {
        match char {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            _ => output.push(char),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::escape_html;

    #[test]
    fn escapes_each_special_character_and_preserves_other() {
        assert_eq!(escape_html("&<>\"'/x"), "&amp;&lt;&gt;&quot;&#x27;&#x2F;x");
    }
}
