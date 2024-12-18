use std::fs;

use regex::Regex;

static mut IS_PART2: bool = false;
pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = get_total_token("./inputs/samples/day13.txt");
    println!("sample: {res}");

    let res = get_total_token("./inputs/puzzles/day13.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = get_total_token("./inputs/samples/day13.txt");
    println!("sample: {res}");

    let res = get_total_token("./inputs/puzzles/day13.txt");
    println!("puzzle: {res}");
}

pub fn get_total_token(file_name: &str) -> i64 {
    parse_input(file_name)
        .into_iter()
        .filter_map(solve_equations)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

const EPSILON: f64 = 10e-3;
pub fn is_int(a: f64) -> bool {
    (a - a.round()).abs() <= EPSILON
}

pub fn solve_equations(
    ((a1, b1, c1), (a2, b2, c2)): ((i64, i64, i64), (i64, i64, i64)),
) -> Option<(i64, i64)> {
    let (a1, b1, c1, a2, b2, c2) = (
        a1 as f64, b1 as f64, c1 as f64, a2 as f64, b2 as f64, c2 as f64,
    );

    let t1 = c2 - a2 * c1 / a1;
    let t2 = b2 - a2 * b1 / a1;

    let b = t1 / t2;
    let a = (c1 - b1 * b) / a1;

    if is_int(a) && is_int(b) {
        let a = a.round() as i64;
        let b = b.round() as i64;

        Some((a, b))
    } else {
        None
    }
}

pub fn parse_input(file_name: &str) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    let re_a = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
    let re_b = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    fs::read_to_string(file_name)
        .unwrap()
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();

            let match_a = re_a.captures(lines.next().unwrap()).unwrap();
            let match_b = re_b.captures(lines.next().unwrap()).unwrap();
            let match_prize = re_prize.captures(lines.next().unwrap()).unwrap();

            let a1 = match_a.get(1).unwrap().as_str().parse().unwrap();
            let a2 = match_a.get(2).unwrap().as_str().parse().unwrap();
            let b1 = match_b.get(1).unwrap().as_str().parse().unwrap();
            let b2 = match_b.get(2).unwrap().as_str().parse().unwrap();

            let mut c1 = match_prize.get(1).unwrap().as_str().parse().unwrap();
            let mut c2 = match_prize.get(2).unwrap().as_str().parse().unwrap();

            if unsafe { IS_PART2 } {
                c1 += 10000000000000;
                c2 += 10000000000000;
            }

            ((a1, b1, c1), (a2, b2, c2))
        })
        .collect()
}
