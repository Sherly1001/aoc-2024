use std::collections::HashMap;

use itertools::iproduct;

use crate::utils::{parse_input::parse_input_vec2d_char, vec_get_bounded::GetBounded};

static mut IS_PART2: bool = false;
pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = sum_scores("./inputs/samples/day10.txt");
    println!("sample: {res}");

    let res = sum_scores("./inputs/puzzles/day10.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = sum_scores("./inputs/samples/day10.txt");
    println!("sample: {res}");

    let res = sum_scores("./inputs/puzzles/day10.txt");
    println!("puzzle: {res}");
}

pub fn count_trailheads(
    curr: (i64, i64),
    map: &Vec<Vec<usize>>,
    locations: &mut HashMap<(i64, i64), usize>,
) {
    let &curr_scale = map
        .get_bounded(curr.0)
        .and_then(|v| v.get_bounded(curr.1))
        .unwrap();

    if curr_scale == 9 {
        *locations.entry(curr).or_default() += 1;
    } else {
        for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (curr.0 + x, curr.1 + y);
            if map
                .get_bounded(next.0)
                .and_then(|v| v.get_bounded(next.1))
                .is_some_and(|&l| l == curr_scale + 1)
            {
                count_trailheads(next, map, locations);
            }
        }
    }
}

pub fn sum_scores(file_name: &str) -> usize {
    let map = get_topo_map(file_name);

    iproduct!(0..map.len(), 0..map[0].len())
        .filter(|&(i, j)| map[i][j] == 0)
        .map(|(i, j)| {
            let mut locations = HashMap::new();
            count_trailheads((i as i64, j as i64), &map, &mut locations);
            if unsafe { IS_PART2 } {
                locations.values().sum::<usize>()
            } else {
                locations.len()
            }
        })
        .sum()
}

pub fn get_topo_map(file_name: &str) -> Vec<Vec<usize>> {
    parse_input_vec2d_char(file_name)
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|i| i.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}
