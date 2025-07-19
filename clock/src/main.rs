use colored::Colorize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        let time = SystemTime::now().duration_since(UNIX_EPOCH);
        let total_seconds = time.unwrap().as_secs();
        let hours = ((total_seconds / 3_600) % 24 + 21) % 24; // GMT-3
        let minutes = (total_seconds % 3_600) / 60;
        let seconds = total_seconds % 60;

        clearscreen::clear().expect("Could not clear screen");
        println!("{}", "hh   mm   ss".bold().yellow());
        println!("{:02} : {:02} : {:02}", hours, minutes, seconds);

        std::thread::sleep(Duration::from_secs(1));
    }
}
