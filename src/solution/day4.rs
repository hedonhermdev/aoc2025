use std::collections::HashSet;

use rayon::vec;

type Pos = (usize, usize);

struct Grid {
    grid: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

fn neighbors(pos: &Pos, rows: usize, cols: usize) -> Vec<Pos> {
        let (x, y) = *pos;
        let x = x as i32;
        let y = y as i32;
        
        let neighs: [(i32, i32); 8] = [
            (x-1, y-1),
            (x-1,  y),
            (x-1, y+1),
            (x , y-1),
            (x , y+1),
            (x+1, y-1),
            (x+1,  y),
            (x+1, y+1),
        ];
        
        neighs.into_iter().filter(|&(x, y)| {
            x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32
        }).map(|(x, y)| {
            (x as usize, y as usize)
        }).collect()
}

impl Grid {
    fn new(input: Vec<Vec<bool>>) -> Self {
        let rows = input.len();
        let cols = if rows > 0 { input[0].len() } else { 0 };
        
        let mut grid = vec![vec![0; cols]; rows];
        
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
                }
            }
        }

        Grid { grid, rows, cols }
    }
    
    fn present(&self, pos: &Pos) -> bool {
        let (x, y) = *pos;
        self.grid[y][x] > 0
    }
    
    fn remove(&mut self, pos: &Pos) {
        let (x, y) = *pos;
        self.grid[y][x] = 0;
        for neigh in self.neighbors(pos) {
            let (nx, ny) = neigh;
            if self.grid[ny][nx] > 0 {
                self.grid[ny][nx] -= 1;
            }
        }
    }
    
    fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        neighbors(pos, self.rows, self.cols)
    }
    
    fn is_accessible(&self, pos: &Pos) -> bool {
        if self.present(pos) {
            let neigh = self.neighbors(pos);
            let count = neigh.iter().filter(|pos| self.present(pos)).count();
            
            count < 4
        } else {
            false
        }
    }
    
    fn remove_all_accessible(&mut self) -> u32 {
        let mut to_remove = vec![];

        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = (col, row);
                if self.is_accessible(&pos) {
                    to_remove.push(pos);
                }
            }
        }
        
        for pos in to_remove.iter() {
            self.remove(pos);
        }
        
        to_remove.len() as u32
    }
}

fn parse_input(input: &str) -> Grid {
    let grid = input
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
        .collect();
            
    Grid::new(grid)
}


pub fn puzzle1(input: &str) -> anyhow::Result<u32> {
    let mut grid = parse_input(input);
    
    Ok(grid.remove_all_accessible())
}

pub fn puzzle2(input: &str) -> anyhow::Result<u32> {
    let mut grid = parse_input(input);
    
    let mut total_removed = 0;
    loop {
        let removed = grid.remove_all_accessible();
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }
    
    Ok(total_removed)
}