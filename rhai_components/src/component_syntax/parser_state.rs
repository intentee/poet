use rhai::LexError;

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
    type Error = LexError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Start),
            1 => Ok(Self::OpeningBracket),
            2 => Ok(Self::Body),
            3 => Ok(Self::BodyExpression),
            4 => Ok(Self::TagLeftAnglePlusWhitespace),
            5 => Ok(Self::TagCloseBeforeNamePlusWhitespace),
            6 => Ok(Self::TagName),
            7 => Ok(Self::TagContent),
            8 => Ok(Self::TagAttributeName),
            9 => Ok(Self::TagAttributeValue),
            10 => Ok(Self::TagAttributeValueString),
            11 => Ok(Self::TagSelfClose),
            unknown_parser_state => Err(LexError::Runtime(format!(
                "Invalid parser state {unknown_parser_state}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use rhai::LexError;

    use super::ParserState;

    #[test]
    fn maps_every_known_state_tag_to_its_state() {
        for parser_state in [
            ParserState::Start,
            ParserState::OpeningBracket,
            ParserState::Body,
            ParserState::BodyExpression,
            ParserState::TagLeftAnglePlusWhitespace,
            ParserState::TagCloseBeforeNamePlusWhitespace,
            ParserState::TagName,
            ParserState::TagContent,
            ParserState::TagAttributeName,
            ParserState::TagAttributeValue,
            ParserState::TagAttributeValueString,
            ParserState::TagSelfClose,
        ] {
            let parser_state_tag = parser_state as i32;

            assert!(
                ParserState::try_from(parser_state_tag)
                    .is_ok_and(|mapped_state| mapped_state as i32 == parser_state_tag)
            );
        }
    }

    #[test]
    fn rejects_unknown_state_tag() {
        assert!(matches!(
            ParserState::try_from(99),
            Err(LexError::Runtime(message)) if message == "Invalid parser state 99"
        ));
    }
}
