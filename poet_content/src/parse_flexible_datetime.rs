use chrono::DateTime;
use chrono::Utc;

use crate::content_error::ContentError;
use crate::date_format::DateFormat;
use crate::date_parse_attempt::DateParseAttempt;

const DATE_PATTERNS: &[&str] = &[
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%d",
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
    "%Y/%m/%d",
    "%d-%m-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M",
    "%d-%m-%Y",
    "%Y-%m-%dT%H:%M:%S%.fZ",
    "%Y-%m-%dT%H:%M:%SZ",
    "%Y-%m-%dT%H:%M:%S",
];

fn date_formats() -> impl Iterator<Item = DateFormat> {
    DATE_PATTERNS
        .iter()
        .flat_map(|pattern| {
            [
                DateFormat::DateTimePattern(pattern),
                DateFormat::DatePattern(pattern),
            ]
        })
        .chain([DateFormat::Rfc3339, DateFormat::Rfc2822])
}

pub fn parse_flexible_datetime(value: &str) -> Result<DateTime<Utc>, ContentError> {
    let mut attempts: Vec<DateParseAttempt> = vec![];

    for date_format in date_formats() {
        match date_format.parse(value) {
            Ok(datetime) => return Ok(datetime),
            Err(source) => attempts.push(DateParseAttempt {
                date_format,
                source,
            }),
        }
    }

    Err(ContentError::InvalidDate {
        attempts,
        value: value.to_owned(),
    })
}
