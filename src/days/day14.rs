use std::fs;

pub fn part1() {
    let res = safety_factor_after("./inputs/samples/day14.txt", &(11, 7), &100);
    println!("sample: {res}");

    let res = safety_factor_after("./inputs/puzzles/day14.txt", &(101, 103), &100);
    println!("puzzle: {res}");
}

pub fn part2() {
    moving_and_draw("./inputs/puzzles/day14.txt", &(101, 103));
}

pub fn moving_and_draw(file_name: &str, map_size: &(i64, i64)) {
    let top_tree = vec![
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '#', '#', '.'],
        vec!['#', '#', '#', '#', '#'],
    ];

    let positions = parse_input(file_name);
    'outer: for time in 0.. {
        let mut picture = vec![vec!['.'; map_size.0 as usize]; map_size.1 as usize];
        for (y, x) in positions.iter().map(|robot| moving(robot, map_size, &time)) {
            picture[x as usize][y as usize] = '#';
        }

        for x in 0..(map_size.1 as usize - 2) {
            for y in 0..(map_size.0 as usize - 4) {
                if picture[x][y..y + 5] == top_tree[0]
                    && picture[x + 1][y..y + 5] == top_tree[1]
                    && picture[x + 2][y..y + 5] == top_tree[2]
                {
                    for row in picture {
                        println!("{}", row.iter().collect::<String>());
                    }
                    println!("Found tree after {time} seconds");
                    break 'outer;
                }
            }
        }
    }
}

pub fn safety_factor_after(file_name: &str, map_size: &(i64, i64), times: &i64) -> usize {
    let positions = parse_input(file_name)
        .iter()
        .map(|robot| moving(robot, map_size, times))
        .collect();
    safety_factor(&positions, map_size)
}

pub fn safety_factor(positions: &Vec<(i64, i64)>, (wide, tall): &(i64, i64)) -> usize {
    let middle_wide = wide / &2;
    let middle_tall = tall / &2;

    positions
        .iter()
        .fold(vec![0, 0, 0, 0], |mut acc, &(x, y)| {
            if x < middle_wide {
                if y < middle_tall {
                    acc[0] += 1;
                } else if y > middle_tall {
                    acc[1] += 1;
                }
            } else if x > middle_wide {
                if y < middle_tall {
                    acc[2] += 1;
                } else if y > middle_tall {
                    acc[3] += 1;
                }
            }

            acc
        })
        .iter()
        .product()
}

pub fn moving(
    ((x, y), (vx, vy)): &((i64, i64), (i64, i64)),
    (wide, tall): &(i64, i64),
    times: &i64,
) -> (i64, i64) {
    let mut x = (x + vx * times) % wide;
    let mut y = (y + vy * times) % tall;

    if x < 0 {
        x += wide;
    }

    if y < 0 {
        y += tall;
    }

    (x, y)
}

pub fn parse_input(file_name: &str) -> Vec<((i64, i64), (i64, i64))> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (_, v) = v.split_once("=").unwrap();

            let (x, y) = p.split_once(",").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();

            (
                (x.parse().unwrap(), y.parse().unwrap()),
                (vx.parse().unwrap(), vy.parse().unwrap()),
            )
        })
        .collect()
}
