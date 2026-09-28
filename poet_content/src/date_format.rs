use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::ParseError;
use chrono::Utc;

#[derive(Debug)]
pub enum DateFormat {
    DatePattern(&'static str),
    DateTimePattern(&'static str),
    Rfc2822,
    Rfc3339,
}

impl DateFormat {
    pub fn parse(&self, value: &str) -> Result<DateTime<Utc>, ParseError> {
        match self {
            Self::DatePattern(pattern) => NaiveDate::parse_from_str(value, pattern)
                .map(|date| date.and_time(NaiveTime::MIN).and_utc()),
            Self::DateTimePattern(pattern) => {
                NaiveDateTime::parse_from_str(value, pattern).map(|datetime| datetime.and_utc())
            }
            Self::Rfc2822 => {
                DateTime::parse_from_rfc2822(value).map(|datetime| datetime.with_timezone(&Utc))
            }
            Self::Rfc3339 => {
                DateTime::parse_from_rfc3339(value).map(|datetime| datetime.with_timezone(&Utc))
            }
        }
    }
}
