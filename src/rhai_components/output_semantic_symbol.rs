use super::expression_reference::ExpressionReference;
use super::tag::Tag;

#[derive(Debug)]
pub enum OutputSemanticSymbol {
    BodyExpression(ExpressionReference),
    Tag(Tag),
    Text(String),
}
