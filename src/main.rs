use std::{env, process::ExitCode};

mod days;
mod utils;

use days::*;

macro_rules! run {
    ($day:expr, [$($days:ident),+ $(,)?]) => {
        match $day.as_str() {
            $(
                stringify!($days) => {
                    println!("============================ part 1 ============================");
                    $days::part1();

                    println!("============================ part 2 ============================");
                    $days::part2();
                }
            ),+
            _ => {
                eprintln!("{} not found.", $day);
                return ExitCode::FAILURE;
            }
        }
    };
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing day to run.");
        return ExitCode::FAILURE;
    }

    let mut day = args[1].clone();
    if !day.starts_with("day") {
        day = format!("day{day}");
    }

    run!(day, [day1, day2, day3, day4, day5, day6]);

    ExitCode::SUCCESS
}
