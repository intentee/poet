use super::tag::Tag;

#[derive(Debug)]
pub enum OutputSemanticSymbol {
    Tag(Tag),
    Text(String),
}
