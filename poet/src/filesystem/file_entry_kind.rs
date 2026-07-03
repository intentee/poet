#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileEntryKind {
    Author,
    Content,
    Other,
    Prompt,
    Shortcode,
}

impl FileEntryKind {
    pub fn is_author(&self) -> bool {
        *self == Self::Author
    }

    pub fn is_content(&self) -> bool {
        *self == Self::Content
    }

    pub fn is_prompt(&self) -> bool {
        *self == Self::Prompt
    }

    pub fn is_shortcode(&self) -> bool {
        *self == Self::Shortcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_prompt_kind_exclusively() {
        let kind = FileEntryKind::Prompt;

        assert!(kind.is_prompt());
        assert!(!kind.is_author());
        assert!(!kind.is_content());
        assert!(!kind.is_shortcode());
    }
}
