use core::str;
use std::str::FromStr;

use aoc_runner_derive::*;

fn digit(num: u64, idx: u32) -> u64 {
    (num / 10u64.pow(idx)) % 10
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Problem<'a> {
    block: Vec<&'a str>,
}

impl<'a> Problem<'a> {
    fn new(block: Vec<&'a str>) -> Self {
        Self { block }
    }

    fn solve1(&self) -> u64 {
        let op = self.block[self.block.len() - 1]
            .trim()
            .parse::<Operator>()
            .unwrap();

        let operands = self.block[..self.block.len() - 1]
            .iter()
            .map(|line| line.trim().parse::<u64>().unwrap());

        match op {
            Operator::Add => operands.sum(),
            Operator::Mul => operands.product(),
        }
    }

    fn solve2(&self) -> u64 {
        let op = self.block[self.block.len() - 1]
            .trim()
            .parse::<Operator>()
            .unwrap();

        let mut operands = vec![];

        for col in 0..self.block[0].len() {
            let mut s = String::new();
            for row in self.block[..self.block.len() - 1].iter() {
                s.push(row.as_bytes()[col] as char);
            }

            operands.push(s.trim().parse::<u64>().unwrap());
        }

        match op {
            Operator::Add => operands.iter().sum(),
            Operator::Mul => operands.iter().product(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Problem<'_>> {
    let lines: Vec<_> = input.lines().collect();

    let operator_line = lines[lines.len() - 1];

    let mut block_ranges = vec![];

    let mut start = 0;
    for i in 1..operator_line.len() {
        if !operator_line.as_bytes()[i].is_ascii_whitespace() {
            block_ranges.push(start..i - 1);
            start = i;
        }
    }
    block_ranges.push(start..operator_line.len());

    let mut problems = vec![];

    for block_range in block_ranges {
        let str_block: Vec<&str> = lines
            .iter()
            .map(|line| &line[block_range.clone()])
            .collect();
        problems.push(Problem::new(str_block));
    }

    problems
}

#[aoc(day6, part1)]
fn puzzle1(input: &str) -> u64 {
    let input = parse_input(input);

    input.iter().map(Problem::solve1).sum()
}

#[aoc(day6, part2)]
fn puzzle2(input: &str) -> u64 {
    let input = parse_input(input);

    input.iter().map(Problem::solve2).sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_puzzle1(){
        assert_eq!(puzzle1(TEST_INPUT),4277556);
    }
    #[test]
    fn test_puzzle2(){
        assert_eq!(puzzle2(TEST_INPUT),3263827);
    }

    }
