use chrono::{prelude::NaiveDateTime, ParseError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DateTime {
    datetime_stamp: NaiveDateTime,
    event_description: String,
}

impl DateTime {
    fn new(datetime: &str, description: &str) -> Result<Self, ParseError> {
        let dt = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")?;
        Ok(Self {
            datetime_stamp: dt,
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
        println!("Event Time: {:?}", self.datetime_stamp.to_string());
        println!("Event Description: {:?}", self.event_description);
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

    let total_events = events
        .into_iter()
        .filter_map(|pair| {
            let (datetime, description) = pair;
            let datetime_and_event = DateTime::new(datetime, description);
            match datetime_and_event {
                Ok(result) => Some(result),
                Err(_) => None,
            }
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
