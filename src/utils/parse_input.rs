use std::{fmt::Debug, fs, str::FromStr};

pub fn parse_input_vec2d<T>(file_name: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn parse_input_vec2d_char(file_name: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
