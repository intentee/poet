#[derive(Clone, Debug, Hash)]
pub struct TagName {
    pub name: String,
}

impl TagName {
    #[must_use]
    pub fn closing_tag(&self) -> String {
        format!("</{}>", self.name)
    }

    #[must_use]
    pub fn is_component(&self) -> bool {
        self.name.chars().next().is_some_and(char::is_uppercase)
    }

    #[must_use]
    pub fn is_void_element(&self) -> bool {
        self.name == "!DOCTYPE"
            || self.name == "area"
            || self.name == "base"
            || self.name == "br"
            || self.name == "col"
            || self.name == "embed"
            || self.name == "hr"
            || self.name == "img"
            || self.name == "input"
            || self.name == "link"
            || self.name == "meta"
            || self.name == "param"
            || self.name == "source"
            || self.name == "track"
            || self.name == "wbr"
    }
}

#[cfg(test)]
mod tests {
    use super::TagName;

    fn tag_name(name: &str) -> TagName {
        TagName {
            name: name.to_owned(),
        }
    }

    #[test]
    fn treats_uppercase_first_character_as_component() {
        assert!(tag_name("Button").is_component());
    }

    #[test]
    fn treats_lowercase_and_empty_names_as_elements() {
        assert!(!tag_name("div").is_component());
        assert!(!tag_name("").is_component());
    }

    #[test]
    fn recognizes_every_void_element() {
        for void_name in [
            "!DOCTYPE", "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
            "param", "source", "track", "wbr",
        ] {
            assert!(tag_name(void_name).is_void_element(), "{void_name}");
        }

        assert!(!tag_name("div").is_void_element());
    }

    #[test]
    fn renders_closing_tag() {
        assert_eq!(tag_name("section").closing_tag(), "</section>");
    }
}
