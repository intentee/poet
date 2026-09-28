/// Taken from Tera: <https://github.com/Keats/tera/blob/master/src/utils.rs>
#[must_use]
pub fn escape_html_attribute(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for char in input.chars() {
        match char {
            '"' => output.push_str("&quot;"),
            _ => output.push(char),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::escape_html_attribute;

    #[test]
    fn escapes_only_double_quote() {
        assert_eq!(escape_html_attribute("\"&<>'/x"), "&quot;&<>'/x");
    }
}
