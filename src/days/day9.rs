use std::fs;

use itertools::Itertools;

pub fn part1() {
    let res = move_file_splited("./inputs/samples/day9.txt");
    println!("sample: {res}");

    let res = move_file_splited("./inputs/puzzles/day9.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    let res = move_file("./inputs/samples/day9.txt");
    println!("sample: {res}");

    let res = move_file("./inputs/puzzles/day9.txt");
    println!("puzzle: {res}");
}

pub fn move_file_splited(file_name: &str) -> usize {
    let mut disk: Vec<_> = fs::read_to_string(file_name)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10))
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            (
                c.next().unwrap().unwrap_or(0) as usize,
                c.next().unwrap().unwrap_or(0) as usize,
            )
        })
        .collect();

    let mut sum = 0;
    let mut idx = 0;

    let (mut i, mut j) = (0, disk.len() - 1);
    while i <= j {
        for _ in 0..disk[i].0 {
            sum += idx * i;
            idx += 1;

            disk[i].0 -= 1;
        }

        while disk[i].1 > 0 && disk[j].0 > 0 {
            sum += idx * j;
            idx += 1;

            disk[i].1 -= 1;
            disk[j].0 -= 1;

            if disk[j].0 == 0 {
                j -= 1;
            }
        }

        i += 1;
    }

    sum
}

pub fn move_file(file_name: &str) -> usize {
    let mut disk_map: Vec<_> = fs::read_to_string(file_name)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(idx, size)| (idx % 2 == 0, idx / 2, size))
        .collect();

    let mut block_idx = disk_map.len() - 1;
    while block_idx > 0 {
        if !disk_map[block_idx].0 {
            block_idx -= 1;
            continue;
        }

        let size = disk_map[block_idx].2;
        let avaiable_empty_block_idx = disk_map
            .iter()
            .take(block_idx)
            .position(|(is_file, _, block_size)| !is_file && block_size >= &size);

        if let Some(idx) = avaiable_empty_block_idx {
            disk_map.insert(idx, disk_map[block_idx]);
            disk_map[block_idx + 1].0 = false;
            disk_map[idx + 1].2 -= size;
        } else {
            block_idx -= 1;
        }
    }

    let mut sum = 0;
    let mut idx = 0;

    for (is_file, id, size) in disk_map {
        if is_file {
            for _ in 0..size {
                sum += idx * id;
                idx += 1;
            }
        } else {
            idx += size;
        }
    }

    sum
}
