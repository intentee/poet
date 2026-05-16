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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::ParserState;

    #[test]
    fn try_from_returns_the_expected_variant_for_each_valid_value() -> Result<()> {
        let expected = [
            (0, ParserState::Start as i32),
            (1, ParserState::OpeningBracket as i32),
            (2, ParserState::Body as i32),
            (3, ParserState::BodyExpression as i32),
            (4, ParserState::TagLeftAnglePlusWhitespace as i32),
            (5, ParserState::TagCloseBeforeNamePlusWhitespace as i32),
            (6, ParserState::TagName as i32),
            (7, ParserState::TagContent as i32),
            (8, ParserState::TagAttributeName as i32),
            (9, ParserState::TagAttributeValue as i32),
            (10, ParserState::TagAttributeValueString as i32),
            (11, ParserState::TagSelfClose as i32),
        ];

        for (input, expected_discriminant) in expected {
            assert!(
                ParserState::try_from(input)
                    .is_ok_and(|state| state as i32 == expected_discriminant)
            );
        }

        Ok(())
    }

    #[test]
    fn try_from_returns_err_for_unknown_value() -> Result<()> {
        assert!(ParserState::try_from(99).is_err());

        Ok(())
    }
}
