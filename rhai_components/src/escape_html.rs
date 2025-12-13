use rhai::ImmutableString;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

/// Taken from Tera: https://github.com/Keats/tera/blob/master/src/utils.rs
pub fn escape_html(input: &str) -> ImmutableString {
    let mut output = SmartString::new_const();

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

    output.into()
}
