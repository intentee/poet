use poet_content::metadata_line_item::MetadataLineItem;
use poet_content::parse_markdown_metadata_line::parse_markdown_metadata_line;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn parses_metadata_line_flags_and_pairs() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        parse_markdown_metadata_line(r#"numbered title:"Main file" lang:'rust' tab:plain"#)?,
        vec![
            MetadataLineItem::Flag {
                name: "numbered".to_owned(),
            },
            MetadataLineItem::Pair {
                name: "title".to_owned(),
                value: "Main file".to_owned(),
            },
            MetadataLineItem::Pair {
                name: "lang".to_owned(),
                value: "rust".to_owned(),
            },
            MetadataLineItem::Pair {
                name: "tab".to_owned(),
                value: "plain".to_owned(),
            },
        ]
    );

    Ok(())
}
