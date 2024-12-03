use std::{collections::HashMap, fs};

#[allow(unused)]
pub fn day1() {
    println!("day1");
    part1();
    part2();
}

pub fn part1() {
    println!("part1");

    let res = get_diff("./inputs/samples/day1.txt");
    println!("sample: {res}");

    let res = get_diff("./inputs/puzzles/day1.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    println!("part2");

    let res = get_similarity_score("./inputs/samples/day1.txt");
    println!("sample: {res}");

    let res = get_similarity_score("./inputs/puzzles/day1.txt");
    println!("puzzle: {res}");
}

pub fn get_diff(file_name: &str) -> i64 {
    let (mut v1, mut v2) = parse_input(file_name);
    v1.sort();
    v2.sort();

    v1.iter()
        .zip(v2)
        .fold(0, |sum, (v1, v2)| sum + (v1 - v2).abs())
}

pub fn get_similarity_score(file_name: &str) -> i64 {
    let (v1, v2) = parse_input(file_name);

    let counting = v2.iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });

    v1.iter()
        .fold(0, |sum, v| sum + v * counting.get(v).unwrap_or(&0))
}

fn parse_input(file_name: &str) -> (Vec<i64>, Vec<i64>) {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .fold((vec![], vec![]), |(mut v1, mut v2), l| {
            let mut iter = l.split_whitespace();
            v1.push(iter.next().unwrap().parse().unwrap());
            v2.push(iter.next().unwrap().parse().unwrap());
            (v1, v2)
        })
}
