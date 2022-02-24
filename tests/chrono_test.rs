use chrono::{DateTime, Duration};
use chrono::prelude::*;

#[test]
fn create_datetime() {
    // let dur = Duration::minutes(20);
    // let local: DateTime<Local> = Local::now();
    // println!("{}", local);
    // println!("{}", local + dur);
    let time = Local::now().time();
    println!("{}", time.hour());
    println!("{}", time.minute());
    println!("{}", time.num_seconds_from_midnight());
}