use std::{env, process};

mod days;
mod utils;

use days::*;

macro_rules! run {
    ($day:expr, [$($days:ident),+ $(,)?]) => {
        match $day.as_str() {
            $(
                stringify!($days) => {
                    println!("============================ part 1 ============================");
                    let timer = std::time::Instant::now();
                    $days::part1();
                    println!("solve time: {}", format!("{:?}", timer.elapsed()));

                    println!("============================ part 2 ============================");
                    let timer = std::time::Instant::now();
                    $days::part2();
                    println!("solve time: {}", format!("{:?}", timer.elapsed()));
                }
            ),+
            _ => {
                eprintln!("{} not found.", $day);
                process::exit(1);
            }
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing day to run.");
        process::exit(1);
    }

    let mut day = args[1].clone();
    if !day.starts_with("day") {
        day = format!("day{:02}", day.parse::<u32>().unwrap());
    }

    run!(
        day,
        [
            day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12,
            day13, day14
        ]
    );
}
