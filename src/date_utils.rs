use std::str::FromStr;

#[cfg(feature = "ssr")]
use surrealdb::Datetime;

#[cfg(not(feature = "ssr"))]
use crate::Datetime;

pub trait FormatDatetime {
    fn format_date(&self) -> String;
    fn format_custom(&self, format_spec: &str) -> String;
    fn ago(&self) -> String;
}

impl FormatDatetime for Datetime {
    fn format_date(&self) -> String {
        format_datetime_custom(self, "%Y-%m-%d %H:%M:%S")
    }

    fn format_custom(&self, format_spec: &str) -> String {
        format_datetime_custom(self, format_spec)
    }

    fn ago(&self) -> String {
        let (_, relative) = format_datetime_local(self);
        relative
    }
}

#[allow(unused)]
pub fn format_date(datetime: &Datetime) -> String {
    #[cfg(feature = "ssr")]
    {
        // For surrealdb::Datetime
        datetime
            .to_string()
            .split('T')
            .next()
            .unwrap_or("")
            .to_string()
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime
        datetime.format("%Y-%m-%d")
    }
}

pub mod time {
    use super::Datetime;
    pub fn now() -> Datetime {
        // Calculate relative time
        let noww = chrono::Utc::now();

        Datetime::from(noww)
    }
}

pub fn from_iso_string(s: &str) -> Datetime {
    let s = s.trim_start_matches("d'").trim_end_matches("'");
    let chronotime = chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now());
    let result = Datetime::from(chronotime);
    result
}

pub fn datetime_from_unix(unixseconds: u64) -> Datetime {
    if unixseconds.to_string().len() > 10 {
        // If the timestamp is in milliseconds, convert to seconds
        let secs = unixseconds / 1000;
        return datetime_from_unix(secs);
    }

    use chrono::{TimeZone, Utc};
    let dt = Utc
        .timestamp_opt(unixseconds as i64, 0)
        .single()
        .unwrap_or_else(|| Utc::now());
    Datetime::from(dt)
}

#[test]
fn test_datetime_from_unix() {
    let unix_time = 1756460403000; // milliseconds
    let datetime = datetime_from_unix(unix_time);
    assert!(datetime.to_string() == "d'2025-08-29T09:40:03Z'");

    let unix_time = 1756460403; // seconds
    let datetime = datetime_from_unix(unix_time);
    assert!(datetime.to_string() == "d'2025-08-29T09:40:03Z'");
}

pub fn datetime_to_unix(datetime: &Datetime) -> Option<u64> {
    let chronotime = parse_surrealdb_datetime_to_chrono(datetime);
    match chronotime {
        Some(dt) => Some(dt.timestamp() as u64),
        None => None,
    }
}

#[test]
fn test_datetime_to_unix() {
    let unix_time = 1756460403000; // milliseconds
    let datetime = datetime_from_unix(unix_time);
    assert!(datetime.to_string() == "d'2025-08-29T09:40:03Z'");
    assert!(datetime_to_unix(&datetime) == Some(1756460403));
}

pub fn parse_surrealdb_datetime_to_chrono(
    datetime: &Datetime,
) -> Option<chrono::DateTime<chrono::Utc>> {
    let datetime_str = datetime.to_string();
    let trimmed = datetime_str.trim_start_matches("d'").trim_end_matches("'");
    match chrono::DateTime::parse_from_rfc3339(trimmed) {
        Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
        Err(_) => None,
    }
}

#[allow(unused)]
pub fn format_date_custom(datetime: &Datetime, format_spec: &str) -> String {
    #[cfg(feature = "ssr")]
    {
        let parsed: chrono::DateTime<chrono::Local> = datetime
            .to_string()
            .parse()
            .unwrap_or_else(|_| chrono::Local::now());

        let formatted = parsed.format(format_spec);
        return formatted.to_string();
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime
        datetime.format(format_spec)
    }
}

#[allow(unused)]
pub fn format_datetime(datetime: &Datetime) -> String {
    let format_spec = "%Y-%m-%d %H:%M:%S";

    #[cfg(feature = "ssr")]
    {
        let parsed: Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> = datetime
            .to_string()
            .trim_start_matches("d'")
            .trim_end_matches("'")
            .parse();

        match parsed {
            Ok(parsed_datetime) => {
                let formatted = parsed_datetime.format(format_spec);
                return formatted.to_string();
            }
            Err(e) => {
                println!("Error parsing datetime: {}", e);
                return "Invalid date".to_string();
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime - format with date and time
        datetime.format(format_spec)
    }
}

/// "%Y-%m-%d %H:%M:%S"
#[allow(unused)]
pub fn format_datetime_custom(datetime: &Datetime, format_spec: &str) -> String {
    #[cfg(feature = "ssr")]
    {
        let parsed: Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> = datetime
            .to_string()
            .trim_start_matches("d'")
            .trim_end_matches("'")
            .parse();

        match parsed {
            Ok(parsed_datetime) => {
                let formatted = parsed_datetime.format(format_spec);
                return formatted.to_string();
            }
            Err(e) => {
                println!("Error parsing datetime: {}", e);
                return "Invalid date".to_string();
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime - format with date and time
        datetime.format(format_spec)
    }
}

pub fn format_datetime_local(datetime: &Datetime) -> (String, String) {
    let unixtime = datetime_to_unix(datetime).unwrap_or(0);
    let output = format_time(unixtime);
    output
}

#[allow(unused)]
pub fn format_time(unix_time: u64) -> (String, String) {
    use chrono::{DateTime, Local, Utc};

    // Convert unix timestamp to DateTime
    let datetime =
        DateTime::<Utc>::from_timestamp(unix_time as i64, 0).unwrap_or_else(|| Utc::now());

    // Convert to local time
    let local_time: DateTime<Local> = datetime.into();
    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Calculate relative time
    let now = Local::now();
    let duration = now.signed_duration_since(local_time);

    let relative = if duration.num_seconds() < 60 {
        format!("{} seconds ago", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        if mins == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", mins)
        }
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if duration.num_days() < 30 {
        let days = duration.num_days();
        if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{} days ago", days)
        }
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    } else {
        let years = duration.num_days() / 365;
        if years == 1 {
            "1 year ago".to_string()
        } else {
            format!("{} years ago", years)
        }
    };

    (formatted_time, relative)
}

pub enum TimeFormatVariant {
    Ago,
    Format(String),
}

#[allow(unused)]
pub fn format_time_iso(timestamp_iso: String, variant: TimeFormatVariant) -> (String, String) {
    use chrono::{DateTime, Local, Utc};

    // Convert unix timestamp to DateTime
    let datetime = DateTime::<Utc>::from_str(&timestamp_iso).unwrap_or_else(|_| Utc::now());

    // Convert to local time
    let local_time: DateTime<Local> = datetime.into();

    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    match variant {
        TimeFormatVariant::Ago => {
            // Calculate relative time
            let now = Local::now();
            let duration = now.signed_duration_since(local_time);

            let relative = if duration.num_seconds() < 60 {
                format!("{} seconds ago", duration.num_seconds())
            } else if duration.num_minutes() < 60 {
                let mins = duration.num_minutes();
                if mins == 1 {
                    "1 minute ago".to_string()
                } else {
                    format!("{} minutes ago", mins)
                }
            } else if duration.num_hours() < 24 {
                let hours = duration.num_hours();
                if hours == 1 {
                    "1 hour ago".to_string()
                } else {
                    format!("{} hours ago", hours)
                }
            } else if duration.num_days() < 30 {
                let days = duration.num_days();
                if days == 1 {
                    "1 day ago".to_string()
                } else {
                    format!("{} days ago", days)
                }
            } else if duration.num_days() < 365 {
                let months = duration.num_days() / 30;
                if months == 1 {
                    "1 month ago".to_string()
                } else {
                    format!("{} months ago", months)
                }
            } else {
                let years = duration.num_days() / 365;
                if years == 1 {
                    "1 year ago".to_string()
                } else {
                    format!("{} years ago", years)
                }
            };

            (formatted_time, relative)
        }
        TimeFormatVariant::Format(format) => {
            let custom = local_time.format(format.as_str()).to_string();

            (formatted_time, custom)
        }
    }
}
