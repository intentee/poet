use super::attribute_value::AttributeValue;
use super::expression_reference::ExpressionReference;

#[derive(Debug)]
pub enum OutputCombinedSymbol {
    BodyExpression(ExpressionReference),
    Text(String),
    TagLeftAngle,
    TagCloseBeforeName,
    TagName(String),
    TagAttributeName(String),
    TagAttributeValue(AttributeValue),
    TagSelfClose,
    TagRightAngle,
}
