use colored::Colorize;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    loop {
        sleep(Duration::from_secs(1));

        let elapsed = start.elapsed().as_secs();
        let hours = elapsed / 3600;
        let minutes = (elapsed % 3600) / 60;
        let seconds = elapsed % 60;

        clearscreen::clear().expect("Could not clear screen");

        println!(
            "{start}{c:~<14}{end}",
            start = ",".purple(),
            c = "~".cyan(),
            end = ".".purple()
        );
        println!(
            "{start} {time} {end}",
            start = "{".red(),
            time = "hh   mm   ss".yellow(),
            end = "}".red()
        );
        println!(
            "{start} {hours:02} : {minutes:02} : {seconds:02} {end}",
            start = "}".red(),
            hours = hours,
            minutes = minutes,
            seconds = seconds,
            end = "{".red()
        );
        println!(
            "{start}{c:~<14}{end}",
            start = "`".purple(),
            c = "~".cyan(),
            end = "'".purple()
        );
    }
}
