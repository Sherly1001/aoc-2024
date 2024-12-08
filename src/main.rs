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
                    $days::part1();

                    println!("============================ part 2 ============================");
                    $days::part2();
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
        day = format!("day{day}");
    }

    run!(day, [day1, day2, day3, day4, day5, day6, day7, day8]);
}
