use crate::SmartStringLazy;

#[derive(Clone, Debug)]
pub enum OutputSymbol {
    BodyExpression,
    Text(SmartStringLazy),
    TagLeftAnglePlusWhitespace,
    TagCloseBeforeNamePlusWhitespace(SmartStringLazy),
    TagName(SmartStringLazy),
    TagPadding,
    TagAttributeName(SmartStringLazy),
    TagAttributeValueExpression,
    TagAttributeValueString(SmartStringLazy),
    TagSelfClose,
    TagRightAngle,
}
