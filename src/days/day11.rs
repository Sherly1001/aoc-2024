use std::{collections::HashMap, fs};

pub fn part1() {
    let res = blinks("./inputs/samples/day11.txt", 25);
    println!("sample: {res}");

    let res = blinks("./inputs/puzzles/day11.txt", 25);
    println!("puzzle: {res}");
}

pub fn part2() {
    let res = blinks("./inputs/samples/day11.txt", 75);
    println!("sample: {res}");

    let res = blinks("./inputs/puzzles/day11.txt", 75);
    println!("puzzle: {res}");
}

pub fn blinks(file_name: &str, times: usize) -> usize {
    let mut cache = HashMap::new();
    fs::read_to_string(file_name)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .map(|v| blink(v, times, &mut cache))
        .sum()
}

pub fn blink(n: usize, k: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let num_stones = if k == 0 {
        1
    } else if n == 0 {
        blink(1, k - 1, cache)
    } else if let Some(&cached) = cache.get(&(n, k)) {
        cached
    } else if let Some((left, right)) = split(n) {
        blink(left, k - 1, cache) + blink(right, k - 1, cache)
    } else {
        blink(n * 2024, k - 1, cache)
    };

    cache.insert((n, k), num_stones);
    num_stones
}

fn split(n: usize) -> Option<(usize, usize)> {
    let num_digits = ((n as f64).log10().floor() + 1.0) as u32;
    if num_digits % 2 == 0 {
        let half = 10_usize.pow(num_digits / 2);
        Some((n / half, n % half))
    } else {
        None
    }
}
