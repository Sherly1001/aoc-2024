use std::fs;

#[allow(unused)]
pub fn day2() {
    println!("day2");
    part1();
    part2();
}

pub fn part1() {
    println!("part1");

    let res = count_safe_reports("./inputs/samples/day2.txt", false);
    println!("sample: {res}");

    let res = count_safe_reports("./inputs/puzzles/day2.txt", false);
    println!("puzzle: {res}");
}

pub fn part2() {
    println!("part2");

    let res = count_safe_reports("./inputs/samples/day2.txt", true);
    println!("sample: {res}");

    let res = count_safe_reports("./inputs/puzzles/day2.txt", true);
    println!("puzzle: {res}");
}

trait IsSafeDiff {
    fn is_safe_diff(self: &Self, last_diff: i64) -> bool;
}

impl IsSafeDiff for i64 {
    fn is_safe_diff(self: &Self, last_diff: i64) -> bool {
        1 <= self.abs() && self.abs() <= 3 && self * last_diff >= 0
    }
}

pub fn is_safe(v: &Vec<i64>) -> bool {
    let mut last_diff = 0;
    for i in 1..v.len() {
        let diff = v[i] - v[i - 1];
        if !diff.is_safe_diff(last_diff) {
            return false;
        }

        last_diff = diff
    }

    true
}

pub fn is_safe_part2(v: &Vec<i64>) -> bool {
    if is_safe(v) {
        return true;
    }

    for i in 0..v.len() {
        let mut v2 = v.clone();
        v2.remove(i);
        if is_safe(&v2) {
            return true;
        }
    }

    false
}

pub fn count_safe_reports(file_name: &str, is_part2: bool) -> u64 {
    let checker = if is_part2 { is_safe_part2 } else { is_safe };

    parse_input(file_name)
        .iter()
        .map(|report| if checker(report) { 1 } else { 0 })
        .sum()
}

pub fn parse_input(file_name: &str) -> Vec<Vec<i64>> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}
