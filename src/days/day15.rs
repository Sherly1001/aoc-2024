use std::{collections::HashSet, fs};

use itertools::iproduct;

use crate::utils::vec_get_bounded::GetBounded;

static mut IS_PART2: bool = false;
pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = sum_gps("./inputs/samples/day15.txt");
    println!("sample: {res}");

    let res = sum_gps("./inputs/puzzles/day15.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = sum_gps("./inputs/samples/day15.txt");
    println!("sample: {res}");

    let res = sum_gps("./inputs/puzzles/day15.txt");
    println!("puzzle: {res}");
}

fn sum_gps(file_name: &str) -> usize {
    let (mut grid, steps) = parse_input(file_name);

    if unsafe { IS_PART2 } {
        grid = extend_map(grid);
    }

    let mut robot_pos = iproduct!(0..grid.len(), 0..grid[0].len())
        .find(|&(i, j)| grid[i][j] == '@')
        .map(|(i, j)| (i as i64, j as i64))
        .unwrap();

    for step in &steps {
        robot_pos = if unsafe { IS_PART2 } {
            moves_part2(&mut grid, &robot_pos, step)
        } else {
            moves(&mut grid, &robot_pos, step)
        };
    }

    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(i, j)| grid[i][j] == 'O' || grid[i][j] == '[')
        .map(|(i, j)| i * 100 + j)
        .sum()
}

fn moves(grid: &mut Vec<Vec<char>>, pos: &(i64, i64), dir: &(i64, i64)) -> (i64, i64) {
    let mut move_able = true;
    let mut trace = vec![];

    let mut next_pos = *pos;
    loop {
        next_pos.0 += dir.0;
        next_pos.1 += dir.1;

        trace.push(next_pos);

        if let Some(c) = grid
            .get_bounded(next_pos.0)
            .and_then(|v| v.get_bounded(next_pos.1))
        {
            if c == &'#' {
                break move_able = false;
            } else if c == &'O' {
                continue;
            }
        }

        break;
    }

    if move_able {
        for i in (1..trace.len()).rev() {
            let (x1, y1) = (trace[i].0 as usize, trace[i].1 as usize);
            let (x2, y2) = (trace[i - 1].0 as usize, trace[i - 1].1 as usize);
            grid[x1][y1] = grid[x2][y2];
        }

        grid[pos.0 as usize][pos.1 as usize] = '.';
        grid[trace[0].0 as usize][trace[0].1 as usize] = '@';

        trace[0]
    } else {
        *pos
    }
}

fn moves_part2(grid: &mut Vec<Vec<char>>, pos: &(i64, i64), dir: &(i64, i64)) -> (i64, i64) {
    let mut trace = vec![];

    if move_able(grid, pos, dir, &mut trace) {
        let mut moved = HashSet::new();
        for i in 0..trace.len() {
            if moved.contains(&trace[i]) {
                continue;
            }

            let (x1, y1) = (trace[i].0 as usize, trace[i].1 as usize);
            let (x2, y2) = ((trace[i].0 + dir.0) as usize, (trace[i].1 + dir.1) as usize);

            grid[x2][y2] = grid[x1][y1];
            grid[x1][y1] = '.';

            moved.insert(trace[i]);
        }

        (pos.0 + dir.0, pos.1 + dir.1)
    } else {
        *pos
    }
}

fn move_able(
    grid: &Vec<Vec<char>>,
    pos: &(i64, i64),
    dir: &(i64, i64),
    trace: &mut Vec<(i64, i64)>,
) -> bool {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let move_able = if let Some(c) = grid
        .get_bounded(next_pos.0)
        .and_then(|v| v.get_bounded(next_pos.1))
    {
        match c {
            '#' => false,
            '[' | ']' => {
                if dir.0 == 0 {
                    move_able(grid, &next_pos, dir, trace)
                } else {
                    let (next_left, next_right) = if c == &'[' {
                        (next_pos, (next_pos.0, next_pos.1 + 1))
                    } else {
                        ((next_pos.0, next_pos.1 - 1), next_pos)
                    };

                    move_able(grid, &next_left, dir, trace)
                        && move_able(grid, &next_right, dir, trace)
                }
            }
            _ => true,
        }
    } else {
        false
    };

    if move_able {
        trace.push(*pos);
    }

    move_able
}

fn extend_map(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => ['.', '.'],
                })
                .collect()
        })
        .collect()
}

fn parse_input(file_name: &str) -> (Vec<Vec<char>>, Vec<(i64, i64)>) {
    let contents = fs::read_to_string(file_name).unwrap();
    let (grid, steps) = contents.split_once("\n\n").unwrap();

    let grid = grid.lines().map(|line| line.chars().collect()).collect();
    let steps = steps
        .chars()
        .filter_map(|c| match c {
            '^' => Some((-1, 0)),
            '>' => Some((0, 1)),
            'v' => Some((1, 0)),
            '<' => Some((0, -1)),
            _ => None,
        })
        .collect();

    (grid, steps)
}
