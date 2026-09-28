use chrono::ParseError;

use crate::date_format::DateFormat;

#[derive(Debug)]
pub struct DateParseAttempt {
    pub date_format: DateFormat,
    pub source: ParseError,
}
