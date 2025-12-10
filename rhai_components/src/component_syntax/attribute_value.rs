use super::expression_reference::ExpressionReference;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Clone, Debug, Hash)]
pub enum AttributeValue {
    Expression(ExpressionReference),
    Text(SmartString),
}
