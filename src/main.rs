use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Utc, TimeZone};
use serde_yaml;


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
                self.calculate() * 1000
            ).unwrap();
        println!("Reckoning: {}", self.reckoning_name);
        println!("Time in readable format: {}", actual_date);
    }

}

fn open_conf() -> Vec<serde_yaml::Value> {
    let file = File::open("./src/conf.yml")
        .expect("ERROR: File doesn't exist");
    let yaml: serde_yaml::Value = serde_yaml::from_reader(&file)
        .expect("ERROR: Could not process yaml");
    let reckonings = yaml
        .get("Reckonings")
        .expect("ERROR: Could not get reckonings from conf.yml");
    let reckonings = reckonings
        .as_sequence()
        .expect("ERROR: Could not get sequence from original yml");
    reckonings.to_vec()
}

fn main() {
    let mappings = open_conf();
    let now = SystemTime::now();
    let seconds = now
        .duration_since(UNIX_EPOCH)
        .expect("ERROR: Cannot get seconds since UNIX epoch")
        .as_secs() as i64;
    for rec in mappings {
        let time = TimeType {
            reckoning_name: String::from(
                                rec["name"]
                                    .as_str()
                                    .unwrap()
                            ),
            timestamp: seconds,
            timediff: rec["diff"].as_i64().unwrap(),
        };
        time.print_time();
    }
}
