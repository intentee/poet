use super::expression_reference::ExpressionReference;

#[derive(Clone, Debug)]
pub enum AttributeValue {
    Expression(ExpressionReference),
    Text(String),
}
