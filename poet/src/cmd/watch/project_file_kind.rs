#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectFileKind {
    Author,
    Content,
    EsbuildMetafile,
    Prompt,
    Shortcode,
}
