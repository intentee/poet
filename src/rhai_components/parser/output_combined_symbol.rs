#[derive(Debug)]
pub enum OutputCombinedSymbol {
    BodyExpressionResult(String),
    Text(String),
    TagLeftAngle,
    TagCloseBeforeName,
    TagName(String),
    TagAttributeName(String),
    TagAttributeValue(String),
    TagSelfClose,
    TagRightAngle,
}
