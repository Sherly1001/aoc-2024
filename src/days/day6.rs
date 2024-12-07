use std::collections::{HashMap, HashSet};

use crate::utils::{parse_input::parse_input_vec2d_char, vec_get_bounded::GetBounded};

pub fn part1() {
    let res = get_visited(&parse_input_vec2d_char("./inputs/samples/day6.txt")).len();
    println!("sample: {res}");

    let res = get_visited(&parse_input_vec2d_char("./inputs/puzzles/day6.txt")).len();
    println!("puzzle: {res}");
}

pub fn part2() {
    let res = count_options("./inputs/samples/day6.txt");
    println!("sample: {res}");

    let res = count_options("./inputs/puzzles/day6.txt");
    println!("puzzle: {res}");
}

pub fn get_visited(map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut dir = (-1, 0);
    let mut start = (0, 0);

    'outer: for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                start = (i as i64, j as i64);
                break 'outer;
            }
        }
    }

    let mut visited = HashSet::new();
    loop {
        visited.insert((start.0 as usize, start.1 as usize));
        let (i, j) = (start.0 + dir.0, start.1 + dir.1);
        match map.get_bounded(i).and_then(|v| v.get_bounded(j)) {
            None => break,
            Some(c) => {
                if c == &'#' {
                    dir = (dir.1, -dir.0);
                    continue;
                }

                start = (i, j);
            }
        }
    }

    visited
}

pub fn count_options(file_name: &str) -> usize {
    let mut map = parse_input_vec2d_char(file_name);
    let visited = get_visited(&map);

    let mut start = (0, 0);
    'outer: for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                start = (i as i64, j as i64);
                break 'outer;
            }
        }
    }

    let mut count = 0;
    for (i, j) in visited {
        let old = map[i][j];
        map[i][j] = '#';
        if has_cycle(start, &map) {
            count += 1;
        }
        map[i][j] = old;
    }

    count
}

pub fn has_cycle(mut start: (i64, i64), map: &Vec<Vec<char>>) -> bool {
    let mut dir = (-1, 0);

    let mut trace = HashMap::new();
    loop {
        let dirs = trace.entry(start).or_insert(HashSet::new());
        if dirs.contains(&dir) {
            return true;
        } else {
            dirs.insert(dir);
        }

        let (i, j) = (start.0 + dir.0, start.1 + dir.1);
        match map.get_bounded(i).and_then(|v| v.get_bounded(j)) {
            None => break,
            Some(c) => {
                if c == &'#' {
                    dir = (dir.1, -dir.0);
                    continue;
                }

                start = (i, j);
            }
        }
    }

    false
}
