#![allow(unused_variables)]
const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "spring";
    let mut points_scored: i32 = 28;
    points_scored = 35;
    let event_time: &str = "06:00";
    let event_time: i32 = 6;
    println!("This {season} season, my favorite team is playing at {event_time}! I expect them to score at least
            {TOUCHDOWN_POINTS} touchdowns and hopefully {points_scored} points!");
    println!(
        "This {0} season, my favorite team is playing at {1}! I expect them to score at least
            {2} touchdowns and hopefully {3} points!",
        season, event_time, TOUCHDOWN_POINTS, points_scored
    );
    let favorite_beverage: &str = "Iced Tea";
}
