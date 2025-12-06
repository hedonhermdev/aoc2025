use core::str;

use aoc_runner_derive::*;

#[allow(dead_code)]
fn parse_input(input: &str) -> &str {
    input
}

fn read_number_row(line: &[u8]) -> u64 {
    let mut num = 0;

    for &b in line {
        num = num * 10 + (b - b'0') as u64;
    }

    num
}

fn read_number_col(lines: &[&[u8]], idx: usize) -> u64 {
    let mut num = 0;

    for line in lines {
        if line[idx] != b' ' {
            num = num * 10 + (line[idx] - b'0') as u64;
        }
    }

    num
}

#[aoc(day6, part1)]
fn puzzle1(input: &str) -> u64 {
    let mut lines = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let op_line = lines.pop().unwrap();

    let mut ops = vec![];
    let mut blocks = vec![];

    let mut start = 0;
    for c in 1..op_line.len() {
        if op_line[c] != b' ' {
            blocks.push(start..c - 1);
            ops.push(op_line[start]);
            start = c;
        }
    }
    blocks.push(start..op_line.len());
    ops.push(op_line[start]);

    let mut res = 0;

    for (block, op) in blocks.into_iter().zip(ops.into_iter()) {
        let mut s = 0;
        for line in &lines {
            let num = read_number_row(line[block.clone()].trim_ascii());
            if op == b'*' {
                if s == 0 {
                    s = num;
                } else {
                    s *= num;
                }
            } else {
                s += num;
            }
        }

        res += s;
    }

    res
}

#[aoc(day6, part2)]
fn puzzle2(input: &str) -> u64 {
    let lines = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut idx = lines[0].len() - 1;
    let op_line = lines.len() - 1;

    let mut operands = vec![];
    let mut res = 0;

    loop {
        operands.push(read_number_col(&lines[..op_line], idx));

        if lines[op_line][idx] != b' ' {
            if lines[op_line][idx] == b'*' {
                res += operands.drain(..).product::<u64>()
            } else {
                res += operands.drain(..).sum::<u64>()
            }
            if idx == 0 {
                // last block
                break;
            } else {
                // skip empty col before next block
                idx -= 1;
            }
        }

        idx -= 1;
    }

    res
}

#[cfg(test)]
const TEST_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

crate::aoc_test!(TEST_INPUT, 4277556, 3263827);
