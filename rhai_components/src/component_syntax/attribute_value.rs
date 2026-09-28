use super::expression_reference::ExpressionReference;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AttributeValue {
    Empty,
    Expression(ExpressionReference),
    Text(String),
}
