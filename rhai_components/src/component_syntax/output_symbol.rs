#[derive(Clone, Debug)]
pub enum OutputSymbol {
    BodyExpression,
    Text(String),
    TagLeftAnglePlusWhitespace,
    TagCloseBeforeNamePlusWhitespace,
    TagName(String),
    TagPadding,
    TagAttributeName(String),
    TagAttributeValueExpression,
    TagAttributeValueString(String),
    TagSelfClose,
    TagRightAngle,
}
