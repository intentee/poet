use super::attribute_value::AttributeValue;
use super::expression_reference::ExpressionReference;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Debug)]
pub enum OutputCombinedSymbol {
    BodyExpression(ExpressionReference),
    Text(SmartString),
    TagLeftAngle,
    TagCloseBeforeName,
    TagName(SmartString),
    TagAttributeName(SmartString),
    TagAttributeValue(AttributeValue),
    TagPadding,
    TagSelfClose,
    TagRightAngle,
}
