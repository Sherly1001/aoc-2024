use std::collections::HashMap;

use itertools::{iproduct, Itertools};

use crate::utils::parse_input::parse_input_vec2d_char;

static mut IS_PART2: bool = false;

pub fn part1() {
    unsafe {
        IS_PART2 = false;
    }

    let res = total_price("./inputs/samples/day12.txt");
    println!("sample: {res}");

    let res = total_price("./inputs/puzzles/day12.txt");
    println!("puzzle: {res}");
}

pub fn part2() {
    unsafe {
        IS_PART2 = true;
    }

    let res = total_price("./inputs/samples/day12.txt");
    println!("sample: {res}");

    let res = total_price("./inputs/puzzles/day12.txt");
    println!("puzzle: {res}");
}

pub fn total_price(file_name: &str) -> usize {
    let grid = parse_input_vec2d_char(file_name);

    let mut blocks = HashMap::new();
    iproduct!(0..grid.len(), 0..grid[0].len()).for_each(|(i, j)| {
        blocks
            .entry(grid[i][j])
            .or_insert(Block::new(grid[i][j]))
            .add(i, j);
    });

    blocks
        .into_values()
        .flat_map(|block| block.split().into_iter().map(|block| block.price()))
        .sum()
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    row: usize,
    col: usize,
    side_up: bool,
    side_down: bool,
    side_left: bool,
    side_right: bool,
}

impl Cell {
    fn sides(&self) -> usize {
        [
            self.side_up,
            self.side_down,
            self.side_left,
            self.side_right,
        ]
        .iter()
        .filter(|&&s| s)
        .count()
    }
}

#[derive(Clone, Debug)]
struct Block {
    id: char,
    cells: Vec<Cell>,
}

impl Block {
    fn new(id: char) -> Self {
        Self { id, cells: vec![] }
    }

    fn add(&mut self, row: usize, col: usize) -> &mut Self {
        let mut cell = Cell {
            row,
            col,
            side_up: true,
            side_down: true,
            side_left: true,
            side_right: true,
        };

        if let Some(c) = self
            .cells
            .iter_mut()
            .find(|c| c.row == row && (c.col).abs_diff(col) == 1)
        {
            let (left, right) = if c.col < col {
                (c, &mut cell)
            } else {
                (&mut cell, c)
            };

            left.side_right = false;
            right.side_left = false;
        }

        if let Some(c) = self
            .cells
            .iter_mut()
            .find(|c| c.col == col && (c.row).abs_diff(row) == 1)
        {
            let (up, down) = if c.row < row {
                (c, &mut cell)
            } else {
                (&mut cell, c)
            };

            up.side_down = false;
            down.side_up = false;
        }

        self.cells.push(cell);
        self
    }

    fn near_by_cell(&self, cell: &Cell) -> bool {
        self.cells.iter().any(|c| {
            (c.row == cell.row && c.col.abs_diff(cell.col) == 1)
                || (c.col == cell.col && c.row.abs_diff(cell.row) == 1)
        })
    }

    fn split(&self) -> Vec<Self> {
        let mut blocks: Vec<Block> = vec![];

        for &cell in &self.cells {
            let mut same_blocks = vec![];

            let mut i = 0;
            while i < blocks.len() {
                if blocks[i].near_by_cell(&cell) {
                    same_blocks.push(blocks.remove(i));
                } else {
                    i += 1;
                }
            }

            let mut cells = vec![cell];
            cells.append(&mut same_blocks.into_iter().flat_map(|b| b.cells).collect_vec());
            blocks.push(Block { id: self.id, cells });
        }

        blocks
    }

    fn area(&self) -> usize {
        self.cells.len()
    }

    fn perimeter(&self) -> usize {
        self.cells.iter().map(|cell| cell.sides()).sum()
    }

    fn sides(&self) -> usize {
        self.perimeter()
            - self
                .cells
                .iter()
                .tuple_combinations()
                .filter_map(|(a, b)| {
                    if a.row == b.row && a.col.abs_diff(b.col) == 1 {
                        Some(
                            [
                                a.side_up == true && b.side_up == true,
                                a.side_down == true && b.side_down == true,
                            ]
                            .iter()
                            .filter(|&&b| b)
                            .count(),
                        )
                    } else if a.col == b.col && a.row.abs_diff(b.row) == 1 {
                        Some(
                            [
                                a.side_left == true && b.side_left == true,
                                a.side_right == true && b.side_right == true,
                            ]
                            .iter()
                            .filter(|&&b| b)
                            .count(),
                        )
                    } else {
                        None
                    }
                })
                .sum::<usize>()
    }

    fn price(&self) -> usize {
        self.area()
            * if unsafe { IS_PART2 } {
                self.sides()
            } else {
                self.perimeter()
            }
    }
}
