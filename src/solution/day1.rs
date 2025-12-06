use aoc_runner_derive::*;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    distance: u32,
}

struct Dial {
    pos: i32,
}

impl Dial {
    fn new() -> Self {
        Dial { pos: 50 }
    }

    fn rotate(&mut self, rotation: &Rotation) -> u32 {
        match rotation.direction {
            Direction::Left => {
                self.pos = (self.pos + 100 - (rotation.distance as i32 % 100)) % 100;
            }
            Direction::Right => {
                self.pos = (self.pos + (rotation.distance as i32 % 100)) % 100;
            }
        }

        assert!(self.pos >= 0 && self.pos < 100);
        self.pos as u32
    }

    fn pos(&self) -> u32 {
        self.pos as u32
    }
}

type Input = Vec<Rotation>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir_char, dist_str) = line.split_at(1);
            let direction = match dir_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction character"),
            };
            let distance: u32 = dist_str.parse().unwrap();

            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

#[aoc(day1, part1)]
fn puzzle1(input: &[Rotation]) -> u32 {
    let mut dial = Dial::new();

    let mut res = 0;

    for rotation in input {
        if dial.rotate(rotation) == 0 {
            res += 1;
        }
    }

    res
}

#[aoc(day1, part2)]
fn puzzle2(input: &[Rotation]) -> u32 {
    let mut dial = Dial::new();

    let mut res = 0;

    for rotation in input {
        let prev_pos = dial.pos();

        let new_pos = dial.rotate(rotation);

        let cycles = rotation.distance / 100;

        res += cycles;

        if prev_pos != 0
            && (new_pos == 0
                || (rotation.direction == Direction::Left && new_pos > prev_pos)
                || (rotation.direction == Direction::Right && new_pos < prev_pos))
        {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
const TEST_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

crate::aoc_test!(TEST_INPUT, 3, 6);
