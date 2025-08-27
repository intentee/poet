#[repr(i32)]
pub enum ParserState {
    Start = 0,
    OpeningBracket = 1,
    Body = 2,
    BodyExpression = 3,
    TagLeftAnglePlusWhitespace = 4,
    TagCloseBeforeNamePlusWhitespace = 5,
    TagName = 6,
    TagContent = 7,
    TagAttributeName = 8,
    TagAttributeValue = 9,
    TagAttributeValueString = 10,
    TagSelfClose = 11,
}

impl TryFrom<i32> for ParserState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParserState::Start),
            1 => Ok(ParserState::OpeningBracket),
            2 => Ok(ParserState::Body),
            3 => Ok(ParserState::BodyExpression),
            4 => Ok(ParserState::TagLeftAnglePlusWhitespace),
            5 => Ok(ParserState::TagCloseBeforeNamePlusWhitespace),
            6 => Ok(ParserState::TagName),
            7 => Ok(ParserState::TagContent),
            8 => Ok(ParserState::TagAttributeName),
            9 => Ok(ParserState::TagAttributeValue),
            10 => Ok(ParserState::TagAttributeValueString),
            11 => Ok(ParserState::TagSelfClose),
            _ => Err(()),
        }
    }
}
