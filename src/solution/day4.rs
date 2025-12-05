use std::collections::HashSet;

use aoc_runner_derive::*;

type Input = Vec<Vec<bool>>;

type Pos = (usize, usize);

struct Grid {
    grid: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    to_remove: HashSet<Pos>,
}

fn neighbors(pos: &Pos, rows: usize, cols: usize) -> Vec<Pos> {
    let (x, y) = *pos;
    let x = x as i32;
    let y = y as i32;

    let neighs: [(i32, i32); 8] = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];

    neighs
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

impl Grid {
    fn new(input: &Input) -> Self {
        let rows = input.len();
        let cols = if rows > 0 { input[0].len() } else { 0 };

        let mut grid = vec![vec![0; cols]; rows];
        let mut to_remove = HashSet::new();

        for r in 0..rows {
            for c in 0..cols {
                if input[r][c] {
                    grid[r][c] = 1;

                    for neigh in neighbors(&(r, c), rows, cols) {
                        let (nx, ny) = neigh;
                        if input[nx][ny] {
                            grid[r][c] += 1;
                        }
                    }
                    if grid[r][c] > 0 && grid[r][c] <= 4 {
                        to_remove.insert((r, c));
                    }
                }
            }
        }

        Grid {
            grid,
            rows,
            cols,
            to_remove,
        }
    }

    fn remove(&mut self, pos: &Pos) -> bool {
        if self.grid[pos.0][pos.1] == 0 {
            false
        } else {
            let (x, y) = *pos;
            self.grid[x][y] = 0;
            for neigh in self.neighbors(pos) {
                let (nx, ny) = neigh;
                if self.grid[nx][ny] > 0 {
                    self.grid[nx][ny] -= 1;
                    if self.grid[nx][ny] > 0 && self.grid[nx][ny] <= 4 {
                        self.to_remove.insert((nx, ny));
                    }
                }
            }

            true
        }
    }

    fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        neighbors(pos, self.rows, self.cols)
    }

    fn remove_all_accessible(&mut self) -> u32 {
        let mut count = 0;

        let to_remove = self.to_remove.drain().collect::<Vec<_>>();

        for pos in to_remove.into_iter() {
            if self.remove(&pos) {
                count += 1;
            }
        }
        count
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn puzzle1(input: &Input) -> u32 {
    let mut grid = Grid::new(input);

    grid.remove_all_accessible()
}

#[aoc(day4, part2)]
pub fn puzzle2(input: &Input) -> u32 {
    let mut grid = Grid::new(input);

    let mut total_removed = 0;
    loop {
        let removed = grid.remove_all_accessible();
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }

    total_removed
}

#[cfg(test)]
const TEST_INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

crate::aoc_test!(TEST_INPUT, 13, 43);