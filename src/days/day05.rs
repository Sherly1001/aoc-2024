use std::{cmp::Ordering, fs};

pub fn part1() {
    let res = count_middles("./inputs/samples/day05.txt");
    println!("sample: {res}");

    let res = count_middles("./inputs/puzzles/day05.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    let res = count_middles_fixed("./inputs/samples/day05.txt");
    println!("sample: {res}");

    let res = count_middles_fixed("./inputs/puzzles/day05.txt");
    println!("puzzle: {res}");
}

fn count_middles(file_name: &str) -> i64 {
    let (orders, updates) = parse_input(file_name);
    updates
        .iter()
        .filter(|v| is_correct(v, &orders))
        .map(|v| v[v.len() / 2])
        .sum()
}

fn count_middles_fixed(file_name: &str) -> i64 {
    let (orders, updates) = parse_input(file_name);
    updates
        .iter()
        .filter(|v| !is_correct(v, &orders))
        .map(|v| fix_incorrect(v, &orders))
        .map(|v| v[v.len() / 2])
        .sum()
}

fn is_correct(v: &Vec<i64>, o: &Vec<(i64, i64)>) -> bool {
    (0..v.len())
        .flat_map(|i| (i + 1..v.len()).map(move |j| (i, j)))
        .all(|(i, j)| !o.contains(&(v[j], v[i])))
}

fn fix_incorrect(v: &Vec<i64>, o: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut v = v.clone();

    v.sort_by(|a, b| {
        if o.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    v
}

fn parse_input(file_name: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let s = fs::read_to_string(file_name).unwrap();
    let (s1, s2) = s.split_once("\n\n").unwrap();

    (
        s1.lines()
            .map(|line| {
                let (x, y) = line.split_once("|").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
        s2.lines()
            .map(|line| line.split(",").map(|v| v.parse().unwrap()).collect())
            .collect(),
    )
}
