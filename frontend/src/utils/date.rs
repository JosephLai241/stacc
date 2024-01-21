//! Contains miscellaneous utilities for formatting dates.

use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::America::Chicago;

/// Format a raw date with the format `%Y-%m-%dT%H:%M:%S%.3f` into an more human-readable format.
pub fn format_date(raw_date: &str) -> String {
    let parsed_datetime = NaiveDateTime::parse_from_str(raw_date, "%Y-%m-%dT%H:%M:%S%.3f")
        .ok()
        .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc));

    let chicago_datetime = parsed_datetime.map(|datetime| datetime.with_timezone(&Chicago));

    let formatted_str =
        chicago_datetime.map(|datetime| datetime.format("%Y/%m/%d %H:%M:%S %Z").to_string());

    if let Some(formatted_date) = formatted_str {
        formatted_date
    } else {
        raw_date.to_string()
    }
}
