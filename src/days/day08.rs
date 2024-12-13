use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

use crate::utils::parse_input::parse_input_vec2d_char;

static mut IS_PART2: bool = false;

pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = get_antinodes(parse_input_vec2d_char("./inputs/samples/day08.txt")).len();
    println!("sample: {res}");

    let res = get_antinodes(parse_input_vec2d_char("./inputs/puzzles/day08.txt")).len();
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = get_antinodes(parse_input_vec2d_char("./inputs/samples/day08.txt")).len();
    println!("sample: {res}");

    let res = get_antinodes(parse_input_vec2d_char("./inputs/puzzles/day08.txt")).len();
    println!("puzzle: {res}");
}

pub fn get_antinodes(grid: Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let (n, m) = (grid.len(), grid[0].len());
    iproduct!(0..n, 0..m)
        .filter(|&(i, j)| grid[i][j] != '.')
        .fold(HashMap::new(), |mut signals, (i, j)| {
            signals
                .entry(grid[i][j])
                .or_insert(vec![])
                .push((i as i64, j as i64));
            signals
        })
        .iter()
        .flat_map(|(_, s)| {
            (0..s.len()).tuple_combinations().flat_map(|(i, j)| {
                let diff = (s[i].0 - s[j].0, s[i].1 - s[j].1);
                if unsafe { IS_PART2 } {
                    let n = n as i64;
                    (-n..n)
                        .map(|k| (s[i].0 + k * diff.0, s[i].1 + k * diff.1))
                        .collect()
                } else {
                    vec![
                        (s[i].0 + diff.0, s[i].1 + diff.1),
                        (s[j].0 - diff.0, s[j].1 - diff.1),
                    ]
                }
            })
        })
        .filter(|&(i, j)| 0 <= i && i < n as i64 && 0 <= j && j < m as i64)
        .collect()
}
