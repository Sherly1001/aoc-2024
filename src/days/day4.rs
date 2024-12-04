use std::fs;

pub fn part1() {
    let res = count_xmas("./inputs/samples/day4.txt");
    println!("sample: {res}");

    let res = count_xmas("./inputs/puzzles/day4.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    let res = count_x_mas("./inputs/samples/day4.txt");
    println!("sample: {res}");

    let res = count_x_mas("./inputs/puzzles/day4.txt");
    println!("puzzle: {res}");
}

pub fn count_xmas(file_name: &str) -> usize {
    let map = parse_input(file_name);

    let mut count = 0;
    for x in -1..2 {
        for y in -1..2 {
            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    if let Some(s) = get_string_from_to(&map, i, j, x, y, "XMAS".len()) {
                        if s.eq("XMAS") {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

pub fn count_x_mas(file_name: &str) -> usize {
    let map = parse_input(file_name);

    let mut count = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] == 'A' {
                let cross1: String = vec![map[i - 1][j - 1], 'A', map[i + 1][j + 1]]
                    .iter()
                    .collect();
                let cross2: String = vec![map[i - 1][j + 1], 'A', map[i + 1][j - 1]]
                    .iter()
                    .collect();

                if (cross1.eq("MAS") || cross1.eq("SAM")) && (cross2.eq("MAS") || cross2.eq("SAM"))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn add(i: usize, x: i64) -> Option<usize> {
    if x > 0 {
        Some(i + (x as usize))
    } else {
        let xabs = x.abs() as usize;
        if i >= xabs {
            Some(i - xabs)
        } else {
            None
        }
    }
}

pub fn get_string_from_to(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    x: i64,
    y: i64,
    n: usize,
) -> Option<String> {
    let mut chars = vec![map[i][j]];
    let mut curr_i = i;
    let mut curr_j = j;

    for _ in 1..n {
        curr_i = add(curr_i, x)?;
        curr_j = add(curr_j, y)?;
        chars.push(*map.get(curr_i)?.get(curr_j)?);
    }

    Some(chars.iter().collect())
}

pub fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
