#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileEntryKind {
    Author,
    #[cfg(feature = "blog")]
    BlogConfig,
    #[cfg(feature = "blog")]
    BlogPost,
    Content,
    Other,
    Prompt,
    Shortcode,
}

impl FileEntryKind {
    pub fn is_author(&self) -> bool {
        *self == Self::Author
    }

    #[cfg(feature = "blog")]
    pub fn is_blog_config(&self) -> bool {
        *self == Self::BlogConfig
    }

    #[cfg(feature = "blog")]
    pub fn is_blog_post(&self) -> bool {
        *self == Self::BlogPost
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
