use poet_content::content_error::ContentError;
use poet_content::parse_markdown_metadata_line::parse_markdown_metadata_line;

#[test]
fn reports_unparsable_metadata_line_remainder() {
    assert!(matches!(
        parse_markdown_metadata_line("numbered title:\"unclosed"),
        Err(ContentError::ParseCodeMetadata { unparsed, .. }) if unparsed == ":\"unclosed"
    ));
}
