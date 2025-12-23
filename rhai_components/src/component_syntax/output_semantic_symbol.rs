use super::expression_reference::ExpressionReference;
use super::tag::Tag;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Debug)]
pub enum OutputSemanticSymbol {
    BodyExpression(ExpressionReference),
    Tag(Tag),
    Text(SmartString),
}
