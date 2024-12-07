use std::fs;

static mut IS_PART2: bool = false;

pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = sum_all_correct("./inputs/samples/day7.txt");
    println!("sample: {res}");

    let res = sum_all_correct("./inputs/puzzles/day7.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = sum_all_correct("./inputs/samples/day7.txt");
    println!("sample: {res}");

    let res = sum_all_correct("./inputs/puzzles/day7.txt");
    println!("puzzle: {res}");
}

pub fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

pub fn eval_equation(val: &i64, acc: i64, terms: &[i64]) -> bool {
    if &acc > val {
        false
    } else if terms.len() == 0 {
        &acc == val
    } else {
        eval_equation(val, acc + terms[0], &terms[1..])
            || eval_equation(val, acc * terms[0], &terms[1..])
            || (unsafe { IS_PART2 } && eval_equation(val, concat(acc, terms[0]), &terms[1..]))
    }
}

pub fn is_correct_equation(val: &i64, terms: &Vec<i64>) -> bool {
    eval_equation(val, terms[0], &terms[1..])
}

pub fn sum_all_correct(file_name: &str) -> i64 {
    parse_input(file_name)
        .iter()
        .filter_map(|(val, terms)| {
            if is_correct_equation(val, terms) {
                Some(val)
            } else {
                None
            }
        })
        .sum()
}

pub fn parse_input(file_name: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let (val, rest) = line.split_once(": ").unwrap();
            (
                val.parse().unwrap(),
                rest.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}
