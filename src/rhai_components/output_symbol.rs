#[derive(Clone, Debug)]
pub enum OutputSymbol {
    BodyExpression,
    Text(String),
    TagLeftAnglePlusWhitespace(String),
    TagCloseBeforeNamePlusWhitespace(String),
    TagName(String),
    TagContent(String),
    TagAttributeName(String),
    TagAttributeValueExpression,
    TagAttributeValueString(String),
    TagSelfClose,
    TagRightAngle,
}
