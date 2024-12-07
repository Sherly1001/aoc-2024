#!/usr/bin/env bash

usage() {
    cat <<END
Run: $0 dayX
Create template for dayX, which including follow files:
    - new files:
        - ./src/days/dayX.rs
        - ./inputs/samples/dayX.txt
        - ./inputs/puzzles/dayX.txt
    - modified:
        - ./src/main.rs
        - ./src/days/mod.rs
END
}

day=$1
[[ -z $day ]] && usage && exit 1
[[ $day != "day*" ]] && day="day$day"

src_dayX="./src/days/$day.rs"
input_sample_dayX="./inputs/samples/$day.txt"
input_puzzle_dayX="./inputs/puzzles/$day.txt"

src_main="./src/main.rs"
src_days_mod="./src/days/mod.rs"

[[ -f $src_dayX ]] && echo "File $src_dayX existed." && exit 1
[[ -f $input_sample_dayX ]] && echo "File $input_sample_dayX existed." && exit 1
[[ -f $input_puzzle_dayX ]] && echo "File $input_puzzle_dayX existed." && exit 1

[[ ! -f $src_main ]] && echo "File $src_main not existed." && exit 1
[[ ! -f $src_days_mod ]] && echo "File $src_days_mod not existed." && exit 1

cat <<END
Create and modifing follow files:
    - new files:
        - $src_dayX
        - $input_sample_dayX
        - $input_puzzle_dayX
    - modified:
        - $src_main
        - $src_days_mod
END

read -p "Are you sure? (y/N) " create
[[ "$create" != "y" ]] && exit 1

cat > $src_dayX <<END
pub fn part1() {
    // let res = run("$input_sample_dayX");
    // println!("sample: {res}");

    // let res = run("$input_puzzle_dayX");
    // println!("puzzle: {res}");
}

pub fn part2() {
    // let res = run("$input_sample_dayX");
    // println!("sample: {res}");

    // let res = run("$input_puzzle_dayX");
    // println!("puzzle: {res}");
}
END

echo > $input_sample_dayX
echo > $input_puzzle_dayX

echo "pub mod $day;" >> $src_days_mod
gsed -i "s/]);/, $day]);/" $src_main
