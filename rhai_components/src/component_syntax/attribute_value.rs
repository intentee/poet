use super::expression_reference::ExpressionReference;
use crate::SmartStringLazy;

#[derive(Clone, Debug, Hash)]
pub enum AttributeValue {
    Expression(ExpressionReference),
    Text(SmartStringLazy),
}
