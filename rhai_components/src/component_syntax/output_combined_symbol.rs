use super::attribute_value::AttributeValue;
use super::expression_reference::ExpressionReference;
use crate::SmartStringLazy;

#[derive(Debug)]
pub enum OutputCombinedSymbol {
    BodyExpression(ExpressionReference),
    Text(SmartStringLazy),
    TagLeftAngle,
    TagCloseBeforeName,
    TagName(SmartStringLazy),
    TagAttributeName(SmartStringLazy),
    TagAttributeValue(AttributeValue),
    TagPadding,
    TagSelfClose,
    TagRightAngle,
}
