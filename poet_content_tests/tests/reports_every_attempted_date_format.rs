use poet_content::content_error::ContentError;
use poet_content::parse_flexible_datetime::parse_flexible_datetime;

#[test]
fn reports_every_attempted_date_format() {
    assert!(matches!(
        parse_flexible_datetime("yesterday"),
        Err(ContentError::InvalidDate { attempts, value })
            if attempts.len() == 28 && value == "yesterday"
    ));
}
