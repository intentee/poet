use poet_content::parse_flexible_datetime::parse_flexible_datetime;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn parses_date_only_as_midnight_utc() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        parse_flexible_datetime("2025-09-12")?.to_rfc3339(),
        "2025-09-12T00:00:00+00:00"
    );

    Ok(())
}
