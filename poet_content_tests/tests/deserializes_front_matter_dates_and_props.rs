use poet_content::content_document_front_matter::ContentDocumentFrontMatter;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn deserializes_front_matter_dates_and_props() -> Result<(), PoetContentTestsError> {
    let front_matter: ContentDocumentFrontMatter = toml::from_str(
        "description = \"d\"\nlast_updated_at = \"2025-09-12\"\nlayout = \"L\"\ntitle = \"t\"\n\n[props]\ncolor = \"red\"\n\n[props.nested]\ndepth = 2",
    )?;

    assert_eq!(
        front_matter
            .last_updated_at
            .map(|last_updated_at| last_updated_at.to_rfc3339()),
        Some("2025-09-12T00:00:00+00:00".to_owned())
    );
    assert_eq!(front_matter.props.len(), 2);

    Ok(())
}
