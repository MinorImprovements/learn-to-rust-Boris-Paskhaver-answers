use chrono::{prelude::*, Duration, ParseError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DateTime {
    datetime_stamp: NaiveDateTime,
    event_description: String,
}

impl DateTime {
    fn new(datetime: &str, description: &str) -> Result<Self, ParseError> {
        let dt = NaiveDateTime::parse_from_str(datetime, "%Y**%m**%d !! %H:%M:%S %z")?;
        Ok(Self {
            datetime_stamp: dt + Duration::hours(4),
            event_description: description.to_string(),
        })
    }

    fn time_gap(&self, other: &Self) {
        let dif = self
            .datetime_stamp
            .signed_duration_since(other.datetime_stamp);
        let sec = dif.num_seconds() % 60;
        let min = dif.num_minutes() % 60;
        let hrs = dif.num_hours();
        println!("Time since last event: {}h, {}m, {}s", hrs, min, sec);
    }

    fn display(&self) {
        println!(
            "Event Time: {:?}",
            self.datetime_stamp.format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        println!("Event Description: {:?}", self.event_description);
    }
}

fn main() {
    let events = vec![
        (
            "2025**04**19 !! 16:00:00 -04:00",
            "Started Rust study session",
        ),
        ("Err", "Err"),
        ("2025**04**20 !! 08:05:30 -04:00", "Made breakfast"),
        ("2025**04**23 !! 22:10:45 -04:00", "Went to bed"),
        ("2025**04**25 !! 09:00:03 -04:00", "Resumed Rust study"),
    ];

    let total_events = events
        .into_iter()
        .filter_map(|pair| {
            let (datetime, description) = pair;
            DateTime::new(datetime, description).ok()
        })
        .collect::<Vec<DateTime>>();

    for event in 0..total_events.len() {
        if event > 0 {
            total_events[event].display();
            total_events[event].time_gap(&total_events[event - 1]);
            println!();
        } else {
            total_events[event].display();
            println!();
        }
    }
}
