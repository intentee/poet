use poet_content::parse_flexible_datetime::parse_flexible_datetime;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn parses_rfc3339_with_offset_as_utc() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        parse_flexible_datetime("2025-09-12T14:30:00+02:00")?.to_rfc3339(),
        "2025-09-12T12:30:00+00:00"
    );

    Ok(())
}
