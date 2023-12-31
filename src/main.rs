use std::{env, time::Instant};

mod helpers;
mod year_2015;
mod year_2016;
mod year_2023;
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let year = args[1].parse::<u16>().unwrap();
        let day = args[2].parse::<u8>().unwrap();
        match year {
            2015 => year_2015::solve(day),
            2016 => year_2016::solve(day),
            2023 => year_2023::solve(day),
            _ => println!("Unknown year {}", year),
        }
    } else {
        println!("Please specify a day and year e.g. year 2023 and day 1 and is `aoc 2023 1`");
    }
    println!("Time taken: {}ms", start.elapsed().as_millis());
}
