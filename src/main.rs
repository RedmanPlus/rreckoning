use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{Utc, TimeZone};


#[derive(Debug)]
struct TimeType {
    reckoning_name: String,
    timestamp: i64,
    timediff: i64,
}

impl TimeType {
    
    fn calculate(&self) -> i64 {
        let actual = self.timestamp + self.timediff;
        actual
    }

    fn print_time(&self) {
        let actual_date = Utc.timestamp_millis_opt(
                self.calculate()
            ).unwrap();
        println!("Reckoning: {}", self.reckoning_name);
        println!("Time in UNIX seconds: {}", self.calculate());
        println!("Time in readable format: {}", actual_date);
    }

}

fn main() {
    let now = SystemTime::now();
    let seconds = now
        .duration_since(UNIX_EPOCH)
        .expect("Cannot get seconds since UNIX epoch")
        .as_secs() as i64;
    println!("{:?}", seconds);
    let time = TimeType {
        reckoning_name: String::from("Gregorian"),
        timestamp: seconds,
        timediff: 0,
    };
    let julian = TimeType {
        reckoning_name: String::from("Julian"),
        timestamp: seconds,
        timediff: -1123000,
    };
    println!("{:?}", time);
    time.print_time();
    julian.print_time();
}
