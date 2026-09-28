use nom::Finish as _;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use crate::content_error::ContentError;
use crate::metadata_line_item::MetadataLineItem;

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|character: char| {
        character.is_alphanumeric() || character == '_' || character == '-'
    })(input)
}

fn unquoted_value(input: &str) -> IResult<&str, &str> {
    take_while1(|character: char| {
        !character.is_whitespace() && character != '"' && character != '\''
    })(input)
}

fn quoted_value(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(
            char('"'),
            take_till(|character| character == '"'),
            char('"'),
        ),
        delimited(
            char('\''),
            take_till(|character| character == '\''),
            char('\''),
        ),
    ))
    .parse(input)
}

fn metadata_line_item(input: &str) -> IResult<&str, MetadataLineItem> {
    alt((
        map(
            separated_pair(identifier, char(':'), alt((quoted_value, unquoted_value))),
            |(name, value)| MetadataLineItem::Pair {
                name: name.to_owned(),
                value: value.to_owned(),
            },
        ),
        map(identifier, |name| MetadataLineItem::Flag {
            name: name.to_owned(),
        }),
    ))
    .parse(input)
}

pub fn parse_markdown_metadata_line(
    metadata_line: &str,
) -> Result<Vec<MetadataLineItem>, ContentError> {
    all_consuming(separated_list0(multispace1, metadata_line_item))
        .parse(metadata_line)
        .finish()
        .map(|(_, metadata_line_items)| metadata_line_items)
        .map_err(|nom_error| ContentError::ParseCodeMetadata {
            metadata_line: metadata_line.to_owned(),
            unparsed: nom_error.input.to_owned(),
        })
}
