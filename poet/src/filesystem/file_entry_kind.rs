#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileEntryKind {
    Content,
    Other,
    Prompt,
    Shortcode,
}

impl FileEntryKind {
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
