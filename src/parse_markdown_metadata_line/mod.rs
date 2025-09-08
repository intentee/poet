pub mod metadata_line_item;

use anyhow::Result;
use anyhow::anyhow;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use crate::parse_markdown_metadata_line::metadata_line_item::MetadataLineItem;

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-')(input)
}

fn unquoted_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '"' && c != '\'')(input)
}

fn quoted_value(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('"'), take_till(|c| c == '"'), char('"')),
        delimited(char('\''), take_till(|c| c == '\''), char('\'')),
    ))
    .parse(input)
}

fn key_value_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(identifier, char(':'), alt((quoted_value, unquoted_value))).parse(input)
}

fn meta_item(input: &str) -> IResult<&str, MetadataLineItem> {
    alt((
        map(key_value_pair, |(key, value)| MetadataLineItem::Pair {
            name: key.to_string(),
            value: value.to_string(),
        }),
        map(identifier, |identifier| MetadataLineItem::Flag {
            name: identifier.to_string(),
        }),
    ))
    .parse(input)
}

pub fn parse_markdown_metadata_line(input: &str) -> Result<Vec<MetadataLineItem>> {
    let (rest, items) = separated_list0(multispace1, meta_item)
        .parse(input)
        .map_err(|err| anyhow!("Failed to parse metadata line: {err}"))?;

    if rest.is_empty() {
        Ok(items)
    } else {
        Err(anyhow!(
            "Unexpected traling characters at the end of markdown metadata line: {rest}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let input = r#"label:"foo bar" class:xD highlighted readonly id:'my-component'"#;
        let result = parse_markdown_metadata_line(input).unwrap();

        assert_eq!(result.len(), 5);

        assert!(result.contains(&MetadataLineItem::Pair {
            name: "label".to_string(),
            value: "foo bar".to_string(),
        }));

        assert!(result.contains(&MetadataLineItem::Pair {
            name: "class".to_string(),
            value: "xD".to_string(),
        }));

        assert!(result.contains(&MetadataLineItem::Pair {
            name: "id".to_string(),
            value: "my-component".to_string(),
        }));

        assert!(result.contains(&MetadataLineItem::Flag {
            name: "highlighted".to_string(),
        }));

        assert!(result.contains(&MetadataLineItem::Flag {
            name: "readonly".to_string(),
        }));
    }
}
