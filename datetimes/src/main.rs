use chrono::prelude::*;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DateTime {
    datetime_stamp: NaiveDateTime,
    event_description: String,
}

impl DateTime {
    fn new(datetime: &str, description: &str) -> Self {
        let dt = NaiveDateTime::parse_and_remainder(datetime, "%Y-%m-%d %H:%M:%S");
        Self {
            datetime_stamp: dt,
            event_description: description.to_string(),
        }
    }
}

fn main() {
    let events = vec![
        ("2025-04-19 20:00:00", "Started Rust study session"),
        ("Err", "Err"),
        ("2025-04-20 12:05:30", "Made breakfast"),
        ("2025-04-23 02:10:45", "Went to bed"),
        ("2025-04-25 13:00:03", "Resumed Rust study"),
    ];
}
