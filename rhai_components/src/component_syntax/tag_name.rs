#[derive(Clone, Debug, Hash)]
pub struct TagName {
    pub name: String,
}

impl TagName {
    pub fn is_component(&self) -> bool {
        self.name
            .chars()
            .next()
            .is_some_and(|first_character| first_character.is_uppercase())
    }

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
    use anyhow::Result;

    use super::TagName;

    #[test]
    fn is_component_returns_true_for_uppercase_first_character() -> Result<()> {
        let tag_name = TagName {
            name: "Button".to_string(),
        };

        assert!(tag_name.is_component());

        Ok(())
    }

    #[test]
    fn is_component_returns_false_for_lowercase_and_for_empty_name() -> Result<()> {
        let lowercase = TagName {
            name: "div".to_string(),
        };
        let empty = TagName {
            name: String::new(),
        };

        assert!(!lowercase.is_component());
        assert!(!empty.is_component());

        Ok(())
    }

    #[test]
    fn is_void_element_recognises_all_void_names_and_rejects_normal_name() -> Result<()> {
        let void_names = [
            "!DOCTYPE", "area", "base", "br", "col", "embed", "hr", "img", "input", "link",
            "meta", "param", "source", "track", "wbr",
        ];

        for void_name in void_names {
            let tag_name = TagName {
                name: void_name.to_string(),
            };

            assert!(tag_name.is_void_element(), "expected {void_name} to be void");
        }

        let non_void = TagName {
            name: "div".to_string(),
        };

        assert!(!non_void.is_void_element());

        Ok(())
    }
}
