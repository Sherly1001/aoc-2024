use std::fs;

#[allow(unused)]
pub fn day3() {
    println!("day3");
    part1();
    part2();
}

pub fn part1() {
    println!("part1");

    let res = mul("./inputs/samples/day3-1.txt");
    println!("sample: {res}");

    let res = mul("./inputs/puzzles/day3.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    println!("part2");

    let res = prog("./inputs/samples/day3-2.txt");
    println!("sample: {res}");

    let res = prog("./inputs/puzzles/day3.txt");
    println!("puzzle: {res}");
}

pub fn mul(file_name: &str) -> i64 {
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(&fs::read_to_string(file_name).unwrap())
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * c.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum()
}

pub fn prog(file_name: &str) -> i64 {
    regex::Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d{1,3}),(\d{1,3})\))")
        .unwrap()
        .captures_iter(&fs::read_to_string(file_name).unwrap())
        .fold((true, 0), |(mut dodo, mut sum), c| {
            if c.get(1).is_some() {
                dodo = true;
            } else if c.get(2).is_some() {
                dodo = false;
            } else if c.get(3).is_some() && dodo {
                sum += c.get(4).unwrap().as_str().parse::<i64>().unwrap()
                    * c.get(5).unwrap().as_str().parse::<i64>().unwrap();
            }
            (dodo, sum)
        })
        .1
}
