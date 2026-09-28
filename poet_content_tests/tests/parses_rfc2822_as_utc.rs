use poet_content::parse_flexible_datetime::parse_flexible_datetime;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn parses_rfc2822_as_utc() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        parse_flexible_datetime("Fri, 12 Sep 2025 14:30:00 +0200")?.to_rfc3339(),
        "2025-09-12T12:30:00+00:00"
    );

    Ok(())
}
