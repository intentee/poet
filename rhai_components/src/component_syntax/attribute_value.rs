use super::expression_reference::ExpressionReference;

#[derive(Clone, Debug, Hash)]
pub enum AttributeValue {
    Expression(ExpressionReference),
    Text(String),
}
