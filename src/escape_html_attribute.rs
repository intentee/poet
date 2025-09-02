/// Taken from Tera: https://github.com/Keats/tera/blob/master/src/utils.rs
pub fn escape_html_attribute(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);

    for char in input.chars() {
        match char {
            // '&' => output.push_str("&amp;"),
            // '<' => output.push_str("&lt;"),
            // '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            // '\'' => output.push_str("&#x27;"),
            // '/' => output.push_str("&#x2F;"),
            _ => output.push(char),
        }
    }

    output
}
