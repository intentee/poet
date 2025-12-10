type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

#[derive(Clone, Debug)]
pub enum OutputSymbol {
    BodyExpression,
    Text(SmartString),
    TagLeftAnglePlusWhitespace,
    TagCloseBeforeNamePlusWhitespace(SmartString),
    TagName(SmartString),
    TagPadding,
    TagAttributeName(SmartString),
    TagAttributeValueExpression,
    TagAttributeValueString(SmartString),
    TagSelfClose,
    TagRightAngle,
}
